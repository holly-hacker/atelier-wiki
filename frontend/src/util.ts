export function containsJapaneseDigit(str: string): boolean {
    return (
        ["０", "１", "２", "３", "４", "５", "６", "７", "８", "９"].find((char) =>
            str.includes(char),
        ) !== undefined
    );
}