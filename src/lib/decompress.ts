import { decompress, init } from "@bokuweb/zstd-wasm";

export function decompressB64(data: string): Uint8Array {
    return Uint8Array.from(atob(data), c => c.charCodeAt(0));
}

export async function decompressZSTD(data: Uint8Array): Promise<Uint8Array> {
    await init()
    return decompress(data);
}