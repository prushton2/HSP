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
        fname: string,
        lname: string,
        pronouns: string,
        number: number,
        hall: string,
        room: number,
        wing: string,
    }
}

export namespace ApiRequestObjects {
    export interface EditStudent {
        uuid: string,
        field: string,
        str_field: string,
        int_field: number,
    }
    
    export interface CreateStudent {
        fname: string,
        lname: string,
        pronouns: string,
        number: number,
        hall: string,
        room: number,
        wing: string,
    }
    
    export interface EditUser {
        uuid: string,
        field: string,
        str_field: string,
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
        uuid: string,
        number: number,
    }
    
    export interface Residencies {
        uuid: string,
        hall: string,
        room: number,
        wing: string,
    }
    
    export interface StudentActivities {
        uuid: string,
        date: number,
        activity: string,
    }
    
    export interface Activities {
        activity: string,
        date: number,
        staff: string[]
    }
    
    export interface Tokens {
        uuid: string,
        signed_up: boolean,
        expiry: number
    }
    
    export interface Users {
        uuid: string,
        fname: string,
        lname: string,
        role: string
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