import React, { useRef } from 'react';

interface SearchbarProps {
    children?:          JSX.Element | JSX.Element[],
    className?:         string,
    classContainer?:    string,
    shortcut?:          string,
    placeholder:        string,
    onChange:           (e: React.ChangeEvent<HTMLInputElement>) => void,
    value:              string
}

export default function Searchbar(props: SearchbarProps) {
    const classN = ( props.className )? ` ${props.className}`:"";
    const classC = ( props.classContainer )? ` ${props.classContainer}`:"";
    const rf = useRef<HTMLInputElement | null>(null);
    if (props.shortcut) {
        document.addEventListener("keydown", (e) => {
            if (e.target != document.body) {
                return;
            }
            if (rf.current && !rf.current.classList.contains("el-focus")) {
                if (e.key == props.shortcut) {
                    e.preventDefault();
                    rf.current.focus();
                    rf.current.classList.add("el-focus");
                }
            }
            // e.stopPropagation();
        });
    }
    return (
        <div className={"relative rounded-md shadow-sm" + classC}>
            <input
                ref={rf}
                value={props.value}
                type="text"
                name="searchbar"
                id="searchbar"
                className={ "relative bg-neutral-900 rounded-md border-0 p-2 pl-8 placeholder:text-gray-400"
                    + " ring-1 ring-inset ring-neutral-700"
                    + " focus:ring-2 focus:ring-inset focus:ring-blue-500 focus:outline-none"
                    + " sm:text-sm sm:leading-6 text-white" + classN }
                placeholder={props.placeholder}
                onBlur={(_) => {
                    if (rf.current) {
                        rf.current.classList.remove("el-focus");
                    }
                }}
                onFocus={(_) => {
                    if (rf.current && !rf.current.classList.contains("el-focus")) {
                        rf.current.classList.add("el-focus");
                    }
                }}
                onChange={(e) => {
                    props.onChange(e);
                }}
            />
            <SearchIcon />
            { (props.shortcut)? <Shortcut sc={props.shortcut}/>:<></> }
        </div>
    )
}

function Shortcut({ sc }: { sc: string }) {
    return (
        <div className='absolute inset-y-0 right-0 flex items-center pr-2'>
            <span className='text-white text-sm
                rounded-md py-1 px-3
                bg-neutral-800'>{sc}</span>
        </div>
    )
}

function SearchIcon() {
    return (
        <div className='absolute text-white inset-y-0 left-0 flex items-center
            pl-2'>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor"
                className="size-5">
                <path strokeLinecap="round" strokeLinejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
            </svg>
        </div>
    )
}
