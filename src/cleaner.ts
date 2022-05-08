const parser = new DOMParser()

export function cleanHtml(input: string): string[] {
    const htmlDoc = parser.parseFromString(input, "text/html")
    const collection = htmlDoc.getElementsByTagName('div')
    const collectionLength = collection.length;
    let cleanHTMLArray = [];
    for (let i = 0; i < collectionLength; i++) {
        const element = collection.item(i)
        const elementText = element.innerText
        cleanHTMLArray.push(elementText)
    }
    return cleanHTMLArray
}