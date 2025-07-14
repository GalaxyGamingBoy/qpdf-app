export function decompressB64(data: string): Uint8Array {
    return Uint8Array.from(atob(data), c => c.charCodeAt(0));
}