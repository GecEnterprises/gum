import * as vscode from "vscode";
import { GumEditorProvider } from "./editor";

export function activate(context: vscode.ExtensionContext) {
  console.log(
    'Congratulations, your extension "gum-code-ext" is now active!'
  );

  const disposable = vscode.commands.registerCommand(
    "gum-code-ext.helloWorld",
    () => {
      vscode.window.showInformationMessage("Hello World from gum-code-ext!");
    }
  );

  const gumEditorProvider = GumEditorProvider.register(context);

  context.subscriptions.push(disposable, gumEditorProvider);
}

export function deactivate() {}
