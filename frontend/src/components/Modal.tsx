import "./Modal.css"
import { useCallback, useRef, useState, type JSX } from "react";
import { useEffect } from "react";

type ShowPromptFn = (title: string, body: JSX.Element) => Promise<boolean>;
type WideModeFn = () => void;

let promptDefault: { show: ShowPromptFn, widen: WideModeFn} = {
    // @ts-ignore
    show: ShowPromptFn => {
        console.warn("PromptContainer is not yet mounted or initialized.");
        return new Promise((res) => { res(false); });
    },
    
    // @ts-ignore
    widen: WideModeFn => {
        console.warn("PromptContainer is not yet mounted or initialized.");
    }
};

export const prompt: { show: ShowPromptFn, widen: WideModeFn } = {
    show: promptDefault.show,
    widen: promptDefault.widen
};


export function Modal(): JSX.Element {
    const [visible, setVisible] = useState<boolean>(false);
    const [wide, setWide] = useState<boolean>(false);
    const [title, setTitle] = useState<string>("")
    const [body, setBody] = useState<JSX.Element>(<></>)

    const resolveRef = useRef<((res: boolean) => void) | null>(null);

    const internalShowPrompt = useCallback((title: string, body: JSX.Element): Promise<boolean> => {
        setVisible(true);
        setTitle(title);
        setBody(body);
        return new Promise((resolve) => {
            resolveRef.current = resolve; // Store the resolve function
        });
    }, []);

    const internalWidePrompt = useCallback(() => {
        setWide(true);
    }, [])

    useEffect(() => {
        prompt.show = internalShowPrompt;
        prompt.widen = internalWidePrompt;
        
        return () => {
            prompt.show = promptDefault.show
            prompt.widen = promptDefault.widen
        }
    }, [internalShowPrompt])

    function handleButton(v: boolean) {
        if (resolveRef.current) {
            resolveRef.current(v)
            resolveRef.current = null
        }
        setVisible(false)
    }

    if (!visible) {
        return <></>
    }

    return (
        <div className="modal-container">
            <div className={`modal-inner-container ${wide ? "wide" : ""}`}>
                <div className="modal-button-x-container">
                    <button onClick={() => {setWide(false); handleButton(false)}}>x</button>
                </div>
                <div className="modal-title">{title}</div>
                <div className="modal-body">{body}</div>
            </div>
        </div>
    );
}