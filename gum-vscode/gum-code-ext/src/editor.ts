import * as vscode from "vscode";

export class GumEditorProvider implements vscode.CustomTextEditorProvider {
  public static register(context: vscode.ExtensionContext): vscode.Disposable {
    const provider = new GumEditorProvider(context);
    const providerRegistration = vscode.window.registerCustomEditorProvider(
      "gum-code-ext.gumEditor",
      provider
    );
    return providerRegistration;
  }

  constructor(private readonly context: vscode.ExtensionContext) {}

  public async resolveCustomTextEditor(
    document: vscode.TextDocument,
    webviewPanel: vscode.WebviewPanel,
    _token: vscode.CancellationToken
  ): Promise<void> {
    webviewPanel.webview.options = { enableScripts: true };

    webviewPanel.webview.html = this.getHtmlForWebview(
      webviewPanel.webview,
      document
    );

    setInterval(() => {
      webviewPanel.webview.postMessage({
        type: "error",
        error: Date.now(),
      });
    }, 10);

    // Listen for messages from the webview
    webviewPanel.webview.onDidReceiveMessage((message) => {
      switch (message.command) {
        case "updateContent":
          this.updateTextDocument(document, message.text);
          break;
      }
    });

    const documentChangeSubscription = vscode.workspace.onDidChangeTextDocument(
      (e) => {
        if (e.document.uri.toString() === document.uri.toString()) {
          webviewPanel.webview.html = this.getHtmlForWebview(
            webviewPanel.webview,
            document
          );
        }
      }
    );

    webviewPanel.onDidDispose(() => {
      documentChangeSubscription.dispose();
    });
  }

  // Helper method to update the text document
  private updateTextDocument(document: vscode.TextDocument, value: string) {
    const edit = new vscode.WorkspaceEdit();
    const fullRange = new vscode.Range(
      document.positionAt(0),
      document.positionAt(document.getText().length)
    );
    edit.replace(document.uri, fullRange, value);
    return vscode.workspace.applyEdit(edit);
  }

  private getHtmlForWebview(
    webview: vscode.Webview,
    document: vscode.TextDocument
  ): string {
    const scriptUri = webview.asWebviewUri(
      vscode.Uri.joinPath(
        this.context.extensionUri,
        "media",
        "dist",
        "index.js"
      )
    );

    // Get the text content of the document
    const content = document.getText();

    return `
      <!DOCTYPE html>
      <html lang="en">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Gum Editor</title>
        <link href="${webview.asWebviewUri(
          vscode.Uri.joinPath(
            this.context.extensionUri,
            "media",
            "dist",
            "index.css"
          )
        )}" rel="stylesheet">
      </head>
      <body>
        <div id="app"></div>
        <script type="module" crossorigin src="${scriptUri}"></script>
        <script>
          const vscode = acquireVsCodeApi();
          window.initialData = ${JSON.stringify(content)};
        </script>
      </body>
      </html>
    `;
  }
}
