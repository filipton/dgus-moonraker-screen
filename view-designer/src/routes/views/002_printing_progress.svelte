<script lang="ts">
    import Header from "$lib/components/Header.svelte";

    export let preview: boolean;
</script>

<Header {preview} />
<div class="w-full h-[calc(100%-2rem)] text-white">
    <div class="text-center font-bold text-2xl mt-1">Currently printing:</div>
    <div
        class="text-center text-xl {preview
            ? 'text-white'
            : 'text-transparent'}"
        title="DATA 2015/20"
    >
        Model name max len..
    </div>

    <hr class="h-px my-2 bg-gray-200 border-0 dark:bg-gray-700" />

    <div class="w-full flex flex-row justify-evenly text-xl">
        <div>
            Nozzle: <span
                class={preview ? "text-white" : "text-transparent"}
                title="DATA 2025/1">198</span
            >
            /
            <span
                class={preview ? "text-white" : "text-transparent"}
                title="DATA 2026/1">200</span
            >
        </div>
        <div>
            Bed: <span
                class={preview ? "text-white" : "text-transparent"}
                title="DATA 2027/1">60</span
            >
            /
            <span
                class={preview ? "text-white" : "text-transparent"}
                title="DATA 2028/1">60</span
            >
        </div>
    </div>

    <div class="flex justify-center mt-4">
        <div
            class="w-5/6 h-6 bg-gray-200 rounded-full dark:bg-gray-700"
            title="DATA 2029/1"
        >
            <div
                class="h-6 bg-blue-600 rounded-full dark:bg-blue-500"
                style="width: {preview ? '45' : '0'}%"
            />
        </div>
    </div>

    <div class="flex h-10 justify-center mt-6 space-x-4">
        <button
            class="rounded-lg border-white border-2 border-solid px-2"
            title="BTN 6"
        >
            <a href="?mod=1">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="h-6 w-atuo inline"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M9 15L3 9m0 0l6-6M3 9h12a6 6 0 010 12h-3"
                    />
                </svg>
                <span class="align-middle text-xl">Back</span>
            </a>
        </button>

        <button
            class="rounded-lg border-white border-2 border-solid px-2 bg-yellow-600"
            title="BTN 7"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="h-6 w-atuo inline"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M15.75 5.25v13.5m-7.5-13.5v13.5"
                />
            </svg>
            <span class="align-middle text-xl">Pause</span>
        </button>

        <button
            class="rounded-lg border-white border-2 border-solid px-2 bg-red-600"
            title="BTN 8"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="h-6 w-atuo inline"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M6 18L18 6M6 6l12 12"
                />
            </svg>
            <span class="align-middle text-xl">Stop</span>
        </button>
    </div>
</div>
