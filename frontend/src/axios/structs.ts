export interface TableStudentInfo {
    uuid: String,
    number: number,
}

export interface TableResidencies {
    uuid: String,
    hall: String,
    room: number,
    wing: String,
}

export interface TableStudentActivities {
    uuid: String,
    date: number,
    activity: String,
}

export interface TableActivities {
    activity: String,
    date: number,
    staff: String[]
}

export interface StudentTablesResponse {
    studentinfo: TableStudentInfo[],
    residence: TableResidencies[],
    // student_activities: TableStudentActivities[],
    // activities: TableActivities[]
    users: TableUsers[],
    tokens: TableTokens[]
}

export interface TableTokens {
    uuid: string,
    token: string,
    signup_hash: string,
    expiry: number
}

export interface EditStudent {
    uuid: String,
    field: String,
    str_field: String,
    int_field: number,
}

export interface FullStudentInfo {
    fname: String,
    lname: String,
    pronouns: String,
    number: number,
    hall: String,
    room: number,
    wing: String,
}

export interface TableUsers {
    uuid: String,
    fname: String,
    lname: String,
    role: String
}

export interface UpdateUser {
    uuid: String,
    field: String,
    str_field: String,
}

export function DefaultAllStudentInfo(): FullStudentInfo {
    return {
        fname: "",
        lname: "",
        pronouns: "",
        number: 0,
        hall: "",
        room: 0,
        wing: "",
        role: "",
    } as FullStudentInfo
}