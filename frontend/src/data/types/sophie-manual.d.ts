export default types;
export namespace types {
    // Item boards
    export type Color = 'R' | 'G' | 'B' | 'W' | 'Y';
    export type ColorOrEmpty = Color | ' ';
    export type ItemBoardRow = `${ColorOrEmpty}${ColorOrEmpty}${ColorOrEmpty}${ColorOrEmpty}${ColorOrEmpty}${ColorOrEmpty}`;

    export type BonusLevel = ' ' | '1' | '2' | '3';
    export type BonusLevelRow = `${BonusLevel}${BonusLevel}${BonusLevel}${BonusLevel}${BonusLevel}${BonusLevel}`;
    export type BonusLevelBoard = [BonusLevelRow, BonusLevelRow, BonusLevelRow, BonusLevelRow, BonusLevelRow, BonusLevelRow];

    export type ItemBoard = {
        "colors": [ItemBoardRow, ItemBoardRow, ItemBoardRow, ItemBoardRow, ItemBoardRow, ItemBoardRow],
        "bonus_levels": [BonusLevelBoard, BonusLevelBoard, BonusLevelBoard],
    };
    export type ItemBoardMap = Record<string, ItemBoard>;

    // Shapes
    export type ShapeIndex = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;
    export type Shape = [
        ShapeIndex, ShapeIndex, ShapeIndex,
        ShapeIndex, ShapeIndex, ShapeIndex,
        ShapeIndex, ShapeIndex, ShapeIndex,
    ];
    export type ShapeMap = Record<string, Shape>;
}
