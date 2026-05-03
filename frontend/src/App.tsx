import "./App.css"
import { useEffect, useState, type JSX } from "react";
import { Toast } from "./components/toast";
import { Http } from "./axios/axios";
import type { Tables } from "./axios/structs";

import { BrowserRouter, Route, Routes } from "react-router";
import Home from './pages/home/Home.tsx'
import Admin from './pages/admin/Admin.tsx';
import Signup from './pages/signup/signup.tsx';
import Nursing from './pages/nursing/nursing.tsx';
import Search from './pages/search/search.tsx';
import Activities from "./pages/activities/activities.tsx";

export default function App(): JSX.Element {
    const [user, setUser] = useState<Tables.Users>({fname: "", lname: "", uuid: "", role: ""} as Tables.Users);
    const [ribbon, setRibbon] = useState<JSX.Element>(<></>)

    useEffect(() => {
        async function init() {
            let user = await Toast.WrapErr(() => Http.Self());
            if(user.is_ok()) {
                setUser(user.into_ok())
            }
        }
        init();
    }, [])

    return <>
        <div className="title">
            <h1 onClick={() => {if(window.location.pathname != "/") {window.location.href = "/"}}}>{GetWindowTitle()}</h1>
            <div className="ribbon">
                {ribbon}
            </div>
        </div>
        <div className='loggedInUser'>{user.fname} <br /> {user.lname}</div>

        <div className="body">
            <BrowserRouter>
                <Routes>
                    <Route path="/" element={<Home user={user}/>} />
                    <Route path="/admin" element={<Admin user={user} ribbon={(e) => setRibbon(e)}/>} />
                    <Route path="/signup" element={<Signup />} />
                    <Route path="/nursing" element={<Nursing />} />
                    <Route path="/search" element={<Search />} />
                    <Route path="/activities" element={<Activities />} />
                </Routes>
            </BrowserRouter>
        </div>
    </>
}

function GetWindowTitle(): string {
    switch (window.location.pathname) {
        case "/":
            return "High School Program";
        case "/admin":
            return "HSP Admin";
        case "/signup":
            return "Signup";
        case "/nursing":
            return "Nursing Numbers";
        case "/search":
            return "Search";
        case "/activities":
            return "Activities";

    }
    return "No Title"
}