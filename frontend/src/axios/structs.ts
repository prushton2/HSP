export namespace ApiResponseObjects {
    export interface AllTables {
        studentinfo: Tables.StudentInfo[],
        residence: Tables.Residencies[],
        student_activities: Tables.StudentActivities[],
        users: Tables.Users[],
        tokens: Tables.Tokens[]
        activities: Tables.Activity[]
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

    export interface CreateActivity {
        name:  string,
        staff: string[],
        dates: number[]
    }

    export interface EditActivity {
        uuid: string,
        name:  string | null,
        staff: string[] | null,
        dates: number[] | null
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

    export interface Activity {
        uuid:  string,
        name:  string,
        staff: string[],
        dates: number[]
    }

    export type AnyTable = StudentInfo | Residencies | StudentActivities | Activity | Users | Tokens;
    export type AnyTableArray = StudentInfo[] | Residencies[] | StudentActivities[] | Activity[] | Users[] | Tokens[];
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