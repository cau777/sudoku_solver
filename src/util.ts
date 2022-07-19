export type AllNull<T> = {
    [P in keyof T]: null;
}