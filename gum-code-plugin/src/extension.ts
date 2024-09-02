// extension.ts
import * as vscode from "vscode";
import { GumEditorProvider } from "./editor";

export function activate(context: vscode.ExtensionContext) {
  console.log(
    'Congratulations, your extension "gum-code-plugin" is now active!'
  );

  const disposable = vscode.commands.registerCommand(
    "gum-code-plugin.helloWorld",
    () => {
      vscode.window.showInformationMessage("Hello World from gum-code-plugin!");
    }
  );

  const gumEditorProvider = GumEditorProvider.register(context);

  context.subscriptions.push(disposable, gumEditorProvider);
}

export function deactivate() {}
