import { useState, type JSX } from "react";
import type { TableUsers, TableActivities, TableResidencies, TableStudentActivities, TableStudentInfo, TableTokens } from "../../axios/structs";

export default function RenderTable({info, tag, select, selected}: {select: (uuid: string) => void, selected: string, info: TableUsers[] | TableTokens[] | TableResidencies[] | TableStudentActivities[] | TableActivities[] | TableStudentInfo[], tag: string}): JSX.Element {
    const [visible, setVisible] = useState<boolean>(false)
    
    let table_rows: JSX.Element[] = [];
    let head: JSX.Element = <></>;

    if (info == undefined) {
        return <></>
    }

    switch (tag) {
        case "student_info":
            head = <tr>
                <th>UUID</th>
                <th>Number</th>
            </tr>
            break;
        case "residencies":
            head = <tr>
                <th>UUID</th>
                <th>hall</th>
                <th>room</th>
                <th>wing</th>
            </tr>
            break;

        case "student_activities":
            head = <tr>
                <th>UUID</th>
                <th>date</th>
                <th>activity</th>
            </tr>
            break;
        case "activities":
            head = <tr>
                <th>activity</th>
                <th>date</th>
                <th>staff</th>
            </tr>
            break;
        

        case "users":
            head = <tr>
                <th>uuid</th>
                <th>first name</th>
                <th>last name</th>
                <th>role</th>
            </tr>
            break;
        case "tokens":
            head = <tr>
                <th>uuid</th>
                <th>token</th>
                <th>signup hash</th>
            </tr>
            break;

    }
    
    info.forEach((row) => {
        let table_row = <></>
        switch (tag) {
            case "student_info":
                table_row = <>
                    <td>{(row as TableStudentInfo).uuid}</td>
                    <td>{(row as TableStudentInfo).number}</td>
                </>
                break;
            case "residencies":
                table_row = <>
                    <td>{(row as TableResidencies).uuid}</td>
                    <td>{(row as TableResidencies).hall}</td>
                    <td>{(row as TableResidencies).room}</td>
                    <td>{(row as TableResidencies).wing}</td>
                </>
                break;

            case "student_activities":
                table_row = <>
                    <td>{(row as TableStudentActivities).uuid}</td>
                    <td>{(row as TableStudentActivities).date}</td>
                    <td>{(row as TableStudentActivities).activity}</td>
                </>
                break;
            case "activities":
                table_row = <>
                    <td>{(row as TableActivities).activity}</td>
                    <td>{(row as TableActivities).date}</td>
                    <td>{(row as TableActivities).staff}</td>
                </>
                break;

            case "users":
                table_row = <>
                    <td>{(row as TableUsers).uuid}</td>
                    <td>{(row as TableUsers).fname}</td>
                    <td>{(row as TableUsers).lname}</td>
                    <td>{(row as TableUsers).role}</td>
                </>
                break;
            case "tokens":
                table_row = <>
                    <td>{(row as TableTokens).uuid}</td>
                    <td>{(row as TableTokens).token}</td>
                    <td>{(row as TableTokens).signup_hash}</td>
                </>
                break;
        }

        if(tag == "activities") {
            table_rows.push(
                <tr>
                    {table_row}
                </tr>
            );
            return;
        } 
        
        if(selected == (row as any).uuid) {
            table_rows = [
                <tr
                    onClick={() => {if (selected == (row as any).uuid) {select("0")} else {select((row as any).uuid)}}}
                    className="highlighted_row"
                >
                    {table_row}
                </tr>,
            ...table_rows]
            return
        }



        table_rows.push(
            <tr 
                onClick={() => {if (selected == (row as any).uuid) {select("0")} else {select((row as any).uuid)}}}
            >
                {table_row}
            </tr>
        )
    });

    return <>
        <h2 onClick={() => setVisible(!visible)}>{formatProperly(tag)}</h2>
        <table>
            <thead onClick={() => setVisible(!visible)}>
                {head}
            </thead>
            {visible ? <tbody>
                {table_rows}
            </tbody> : <></>}
        </table>
    </>
}

function formatProperly(s: string): string {
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