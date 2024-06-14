import { momentum } from 'ldrs';


export function Loading() {
    momentum.register();
    return (
        <div className="fixed w-max h-max top-1/2 left-1/2 translate-x-[-50%] translate-y-[-50%]">
            <l-momentum
                size="75"
                speed="1.1"
                color="white"
            ></l-momentum>
        </div>
    )
}
