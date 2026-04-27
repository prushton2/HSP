export namespace ApiResponseObjects {
    export interface AllTables {
        studentinfo: Tables.StudentInfo[],
        residence: Tables.Residencies[],
        student_activities: Tables.StudentActivities[],
        activities: Tables.Activities[]
        users: Tables.Users[],
        tokens: Tables.Tokens[]
    }
    
    export interface FullStudent {
        fname: String,
        lname: String,
        pronouns: String,
        number: number,
        hall: String,
        room: number,
        wing: String,
    }
}

export namespace ApiRequestObjects {
    export interface EditStudent {
        uuid: String,
        field: String,
        str_field: String,
        int_field: number,
    }
    
    export interface CreateStudent {
        fname: String,
        lname: String,
        pronouns: String,
        number: number,
        hall: String,
        room: number,
        wing: String,
    }
    
    export interface EditUser {
        uuid: String,
        field: String,
        str_field: String,
    }

    export interface SearchStudent {
        fname:  string | null,
        lname:  string | null,
        number: number | null,
        hall:   string | null,
        room:   number | null,
    }
}

export namespace Tables {
    export interface StudentInfo {
        uuid: String,
        number: number,
    }
    
    export interface Residencies {
        uuid: String,
        hall: String,
        room: number,
        wing: String,
    }
    
    export interface StudentActivities {
        uuid: String,
        date: number,
        activity: String,
    }
    
    export interface Activities {
        activity: String,
        date: number,
        staff: String[]
    }
    
    export interface Tokens {
        uuid: string,
        token: string,
        signup_hash: string,
        expiry: number
    }
    
    export interface Users {
        uuid: String,
        fname: String,
        lname: String,
        role: String
    }

    export type AnyTable = StudentInfo | Residencies | StudentActivities | Activities | Users | Tokens;
    export type AnyTableArray = StudentInfo[] | Residencies[] | StudentActivities[] | Activities[] | Users[] | Tokens[];
}

export function DefaultAllStudentInfo(): ApiRequestObjects.CreateStudent {
    return {
        fname: "",
        lname: "",
        pronouns: "",
        number: 0,
        hall: "",
        room: 0,
        wing: "",
        role: "",
    } as ApiRequestObjects.CreateStudent
}