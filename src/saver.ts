import { fs } from "@tauri-apps/api";

export async function saveFile(filePath: string, fileContent: string) {
    const options: fs.FsTextFileOption = {
        contents: fileContent,
        path: filePath
    } 

    await fs.writeFile(options)
}