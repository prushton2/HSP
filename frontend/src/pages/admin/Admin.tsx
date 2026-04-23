import { useEffect, useState, type JSX } from 'react'
import './Admin.css'
import { GetAllStudentInfo, HttpCreateStudent } from '../../axios/axios'
import { type CreateStudent, type StudentTablesResponse, type TableActivities, type TableResidencies, type TableStudentActivities, type TableStudentInfo } from '../../axios/structs';

function Admin() {
    const [studentInfo, setStudentInfo] = useState<StudentTablesResponse>({} as StudentTablesResponse);
    const [selectedUUID, setSelectedUUID] = useState<string>("");

    useEffect(() => {
        async function init() {
            let info = await GetAllStudentInfo()
            setStudentInfo(info);
            console.log(info);
            console.log(typeof info);
        }
        init();
    }, [])

    return (
    <>
        <div className="title">
        <h1>HSP Admin</h1>
        </div>

        <div className="tables">
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.student_info} tag="student_info" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.residencies} tag="residencies" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.student_activities} tag="student_activities" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.activities} tag="activities" />

        </div>
        <div className="forms">
            <CreateStudent />
        </div>
    </>
    )
}

export default Admin


function RenderTable({info, tag, select, selected}: {select: (uuid: string) => void, selected: string, info: TableResidencies[] | TableStudentActivities[] | TableActivities[] | TableStudentInfo[], tag: string}): JSX.Element {
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
                <th>role</th>
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

    }
    
    info.forEach((row) => {
        console.log(typeof row);
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
                    <td>{(row as TableResidencies).role}</td>
                    
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
        }

        if(tag == "activities") {
            table_rows.push(
                <tr>
                    {table_row}
                </tr>
            );
        } else {
            table_rows.push(
                <tr 
                    onClick={() => {if (selected == (row as any).uuid) {select("0")} else {select((row as any).uuid)}}}
                    className={selected == (row as any).uuid ? "highlighted_row" : ""}
                >
                    {table_row}
                </tr>
            )
        }
    });

    return <>
    <h2 onClick={() => setVisible(!visible)}>{formatProperly(tag)}</h2>
    {visible ? <table>
        <thead>
            {head}
        </thead>
        <tbody>
            {table_rows}
        </tbody>
    </table> : <></>}
    </>
}

function CreateStudent(): JSX.Element {
    const [state, setState] = useState<CreateStudent>({room: 0} as CreateStudent);

    return <>
        <table>
        <h3>Create Student</h3>
        <tbody>
            <tr><td>fname </td><td><input onChange={(e) => setState({...state, fname:  e.target.value})} /> </td></tr>
            <tr><td>lname </td><td><input onChange={(e) => setState({...state, lname:  e.target.value})} /> </td></tr>
            <tr><td>number</td><td><input onChange={(e) => setState({...state, number: parseInt(e.target.value)})} type="number" />  </td></tr>
            <tr><td>hall  </td><td><input onChange={(e) => setState({...state, hall:   e.target.value})} /> </td></tr>
            <tr><td>room  </td><td><input onChange={(e) => setState({...state, room:   parseInt(e.target.value)})} type="number" /> </td></tr>
            <tr><td>wing  </td><td><input onChange={(e) => setState({...state, wing:   e.target.value})} /> </td></tr>
            <tr><td>role  </td><td><input onChange={(e) => setState({...state, role:   e.target.value})} /> </td></tr>
            <tr><td></td><td><button onClick={() => HttpCreateStudent(state)}>Create</button></td></tr>
        </tbody>
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