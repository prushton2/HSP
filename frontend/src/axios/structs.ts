export interface TableStudentInfo {
    uuid: String,
    number: number,
}

export interface TableResidencies {
    uuid: String,
    hall: String,
    room: number,
    wing: String,
    role: String,
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
    student_info: TableStudentInfo[],
    residencies: TableResidencies[],
    student_activities: TableStudentActivities[],
    activities: TableActivities[]
}

export interface CreateStudent {
    fname: String,
    lname: String,
    number: number,
    hall: String,
    room: number,
    wing: String,
    role: String,
}

export interface EditStudent {
    uuid: String,
    field: String,
    str_field: String,
    int_field: number,
}