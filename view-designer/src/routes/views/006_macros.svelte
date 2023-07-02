<script lang="ts">
    import Header from "$lib/components/Header.svelte";

    export let preview: boolean;
</script>

<Header {preview} />
<div class="w-full h-[calc(100%-2rem)] text-white flex flex-col">
    <div class="w-full flex flex-row space-x-2">
        <div class="flex flex-col w-full px-2 pt-2 space-y-2">
            <button class="menu_button w-full h-10"
                >{preview ? "MACRO 1" : ""}</button
            >
            <button class="menu_button w-full h-10"
                >{preview ? "MACRO 2" : ""}</button
            >
            <button class="menu_button w-full h-10"
                >{preview ? "MACRO 3" : ""}</button
            >
            <button class="menu_button w-full h-10"
                >{preview ? "MACRO 4" : ""}</button
            >
        </div>

        <div class="flex w-16 flex-col px-2 pt-2 space-y-2">
            <button class="menu_button w-full h-1/2">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="w-full h-full"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M8.25 6.75L12 3m0 0l3.75 3.75M12 3v18"
                    />
                </svg>
            </button>
            <button class="menu_button w-full h-1/2">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="w-full h-full"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M15.75 17.25L12 21m0 0l-3.75-3.75M12 21V3"
                    />
                </svg>
            </button>
        </div>
    </div>

    <div class="flex w-full mb-2 mt-auto pl-2">
        <button class="menu_button w-24 h-8">Back</button>
    </div>
</div>

<style lang="postcss">
    .menu_button {
        @apply rounded-lg border-2 border-white border-solid bg-gray-800;
    }
</style>
