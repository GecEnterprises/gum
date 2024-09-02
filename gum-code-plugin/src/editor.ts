// editor.ts
import * as vscode from "vscode";

export class GumEditorProvider implements vscode.CustomTextEditorProvider {
  public static register(context: vscode.ExtensionContext): vscode.Disposable {
    const provider = new GumEditorProvider(context);
    const providerRegistration = vscode.window.registerCustomEditorProvider(
      "gum-code-plugin.gumEditor",
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
    // Set the webview's initial HTML content
    webviewPanel.webview.options = { enableScripts: true };

    // Update the content based on the document's content
    webviewPanel.webview.html = this.getHtmlForWebview(document);

    // Listen for changes in the text document and update the webview
    const documentChangeSubscription = vscode.workspace.onDidChangeTextDocument(
      (e) => {
        if (e.document.uri.toString() === document.uri.toString()) {
          webviewPanel.webview.html = this.getHtmlForWebview(document);
        }
      }
    );

    // Dispose of the subscription when the webview panel is disposed of
    webviewPanel.onDidDispose(() => {
      documentChangeSubscription.dispose();
    });
  }

  private getHtmlForWebview(document: vscode.TextDocument): string {
    const content = document.getText();
    // Example HTML content; replace this with your custom editor content
    return `
      <!DOCTYPE html>
      <html lang="en">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Gum Editor</title>
      </head>
      <body>
        <textarea style="width: 100%; height: 100vh;">${content}</textarea>
      </body>
      </html>
    `;
  }
}
