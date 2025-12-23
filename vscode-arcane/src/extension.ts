import * as path from 'path';
import * as fs from 'fs';
import { workspace, ExtensionContext, window } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    // Get the server path from settings or use default
    const config = workspace.getConfiguration('arcane');
    let serverPath = config.get<string>('serverPath');

    if (!serverPath || serverPath === '') {
        // Try to find the server in common locations
        const possiblePaths = [
            // Development: built with cargo
            path.join(context.extensionPath, '..', 'target', 'release', 'arcane-lsp'),
            path.join(context.extensionPath, '..', 'target', 'debug', 'arcane-lsp'),
            // Installed globally
            'arcane-lsp',
            // In the extension folder
            path.join(context.extensionPath, 'bin', 'arcane-lsp'),
        ];

        for (const p of possiblePaths) {
            if (p === 'arcane-lsp' || fs.existsSync(p)) {
                serverPath = p;
                break;
            }
        }
    }

    if (!serverPath) {
        window.showErrorMessage(
            'Arcane LSP server not found. Please build it with `cargo build --release` or set arcane.serverPath in settings.'
        );
        return;
    }

    const serverExecutable: Executable = {
        command: serverPath,
        options: {
            env: process.env,
        },
    };

    const serverOptions: ServerOptions = {
        run: serverExecutable,
        debug: serverExecutable,
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'arcane' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.arc'),
        },
    };

    client = new LanguageClient(
        'arcaneLsp',
        'Arcane Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client (and server)
    client.start();

    context.subscriptions.push({
        dispose: () => {
            if (client) {
                client.stop();
            }
        },
    });
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
