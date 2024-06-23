type Base64 = string

export class FileReaderTool {
  public static async AsBase64(file: File): Promise<Base64> {
    return new Promise<string>((resolve) => {
      const reader = new FileReader()
      reader.readAsDataURL(file)
      reader.onload = async () => {
        const image = reader.result
        return resolve(String(image))
      }
    })
  }
}
