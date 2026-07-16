import * as path from 'path';
import {
    workspace,
    ExtensionContext,
    window,
    commands,
    StatusBarAlignment,
    StatusBarItem,
} from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
    RevealOutputChannelOn,
    State,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;
let statusBar: StatusBarItem | undefined;

export function activate(context: ExtensionContext): void {
    statusBar = window.createStatusBarItem(StatusBarAlignment.Left, 10);
    statusBar.text = '$(sync~spin) OxiLean';
    statusBar.tooltip = 'OxiLean Language Server starting…';
    statusBar.show();
    context.subscriptions.push(statusBar);

    startLanguageServer(context);

    context.subscriptions.push(
        commands.registerCommand('oxilean.restartServer', async () => {
            await stopLanguageServer();
            startLanguageServer(context);
        })
    );
}

function startLanguageServer(context: ExtensionContext): void {
    const config = workspace.getConfiguration('oxilean');
    const serverPath: string = config.get('serverPath', 'oxilean');
    const serverArgs: string[] = config.get('serverArgs', ['--lsp']);

    const serverOptions: ServerOptions = {
        run: {
            command: serverPath,
            args: serverArgs,
            transport: TransportKind.stdio,
        },
        debug: {
            command: serverPath,
            args: [...serverArgs, '--verbose'],
            transport: TransportKind.stdio,
        },
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'oxilean' },
        ],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.{oxilean,lean}'),
        },
        outputChannelName: 'OxiLean Language Server',
        revealOutputChannelOn: RevealOutputChannelOn.Error,
    };

    client = new LanguageClient(
        'oxilean',
        'OxiLean Language Server',
        serverOptions,
        clientOptions
    );

    client.onDidChangeState((event) => {
        if (!statusBar) return;
        if (event.newState === State.Running) {
            statusBar.text = '$(check) OxiLean';
            statusBar.tooltip = 'OxiLean Language Server running';
        } else if (event.newState === State.Starting) {
            statusBar.text = '$(sync~spin) OxiLean';
            statusBar.tooltip = 'OxiLean Language Server starting…';
        } else {
            statusBar.text = '$(error) OxiLean';
            statusBar.tooltip = 'OxiLean Language Server stopped';
        }
    });

    context.subscriptions.push(client);
    client.start().catch((err: Error) => {
        window.showErrorMessage(`OxiLean: Failed to start language server: ${err.message}`);
    });
}

async function stopLanguageServer(): Promise<void> {
    if (client) {
        await client.stop();
        client = undefined;
    }
}

export async function deactivate(): Promise<void> {
    await stopLanguageServer();
}

// Suppress unused import warning — 'path' is available for future use
// (e.g. resolving serverPath relative to extension root)
void path;
