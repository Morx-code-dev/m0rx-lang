const vscode = require('vscode');

function activate(context) {
    console.log('M0RX Language extension is active!');

    const statusBar = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Left, 100
    );
    statusBar.text = '$(code) M0RX';
    statusBar.tooltip = 'M0RX Language v0.1.0';
    statusBar.show();

    let runCmd = vscode.commands.registerCommand('m0rx.runFile', function() {
        const editor = vscode.window.activeTextEditor;
        if (editor) {
            const file = editor.document.fileName;
            const terminal = vscode.window.createTerminal('M0RX');
            terminal.show();
            terminal.sendText(`morxc run "${file}"`);
        }
    });

    let fmtCmd = vscode.commands.registerCommand('m0rx.formatFile', function() {
        const editor = vscode.window.activeTextEditor;
        if (editor) {
            const file = editor.document.fileName;
            const terminal = vscode.window.createTerminal('M0RX Formatter');
            terminal.show();
            terminal.sendText(`morxfmt "${file}"`);
        }
    });

    let lintCmd = vscode.commands.registerCommand('m0rx.lintFile', function() {
        const editor = vscode.window.activeTextEditor;
        if (editor) {
            const file = editor.document.fileName;
            const terminal = vscode.window.createTerminal('M0RX Linter');
            terminal.show();
            terminal.sendText(`morxlint "${file}"`);
        }
    });

    context.subscriptions.push(statusBar, runCmd, fmtCmd, lintCmd);
}

function deactivate() {}

module.exports = { activate, deactivate };
