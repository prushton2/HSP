import { useEffect, useState } from 'react'
import './App.css'
import type { Tables } from './axios/structs'
import { Http } from './axios/axios';
import { Toast } from './components/toast';
import { RoleGE } from './components/Role';

function App() {
    const [user, setUser] = useState<Tables.Users>({fname: "", lname: "", uuid: "", role: ""} as Tables.Users);

    useEffect(() => {
        async function init() {
            let user = await Toast.WrapErr(() => Http.Self());
            if(user.is_ok()) {
                setUser(user.into_ok())
            }
        }
        init();
    })

    return (
        <>
        <div className="title">
            <h1>HSP {user.fname} </h1>
        </div>

        <div className="body">
            { RoleGE(user.role, "Admin") ?
                <div className="titleButton" onClick={() => window.location.href="/admin"}>Admin</div>
                : <></>
            }
            <div className="titleButton" onClick={() => window.location.href="/nursing"}>Nursing Numbers</div>
            <div className="titleButton" onClick={() => window.location.href="/activities"}>Activities</div>
            <div className="titleButton" onClick={() => window.location.href="/search"}>Search</div>
        </div>

        </>
    )
}

export default App
