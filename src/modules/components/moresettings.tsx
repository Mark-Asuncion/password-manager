import { useState } from "react";
import { DropDown, ButtonProps } from "./button";

interface MoreSettingsProps extends ButtonProps {
    options: {
        name: string,
        cb:   () => void
    }[]
}

export default function MoreSettings(props: MoreSettingsProps) {
    const [showOpt, setShowOpt] = useState(false);

    return (
        <DropDown
            className="relative"
            onClick={(e) => {
                props.onClick(e);
                setShowOpt(prev => !prev);
            }}>
            <div>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor"
                    className="size-6 my-auto">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" />
                </svg>
            </div>
            {
                (showOpt)?
                <div className="py-2 flex flex-col bg-neutral-800 rounded-md shadow-black shadow-md absolute top-10 left-[-50%] z-10">
                        {
                            props.options.map((v,i) => {
                                return (
                                    <div
                                        key={i}
                                        className="text-white text-center text-md hover:bg-neutral-700 py-1 px-3"
                                        onClick={(_) => v.cb()}
                                    >{v.name}</div>
                                )
                            })
                        }
                </div>
                :<></>
            }
        </DropDown>
    )
}
