export function formatProperly(s: string): string {
    s = s.replaceAll("_", " ");
    s = s.charAt(0).toUpperCase() + s.substring(1);

    let news = s

    for(let i = 0; i < s.length-1; i++) {
        if (s[i] == " ") {
            news = s.substring(0, i+1) + s[i+1].toUpperCase() + s.substring(i+2);
        }
    }

    return news;
}