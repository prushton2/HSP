export function RoleGE(a: string, b: string): boolean {
    switch(a) {
        case "Owner":
            return true;
        case "Admin":
            return b != "Owner";
        case "Staff":
            return b == "Staff";
    }
    return false;
}