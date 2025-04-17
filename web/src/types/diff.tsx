export interface Diff {
    added: [Record<string, any>];
    removed: [Record<string, any>];
    unchanged: [Record<string, any>];
}