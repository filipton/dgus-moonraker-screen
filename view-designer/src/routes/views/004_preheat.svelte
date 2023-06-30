<script lang="ts">
    import Header from "$lib/components/Header.svelte";

    export let preview: boolean;
</script>

<Header {preview} />
<div class="w-full h-[calc(100%-2rem)] text-white">
    <div class="flex w-full h-full p-4">
        <div class="flex w-full flex-col mx-auto my-auto">
            <button class="w-full menu_button py-1.5" title="BTN 10">
                PLA
            </button>
            <button class="w-full menu_button mt-2 py-1.5" title="BTN 11">
                COOLDOWN
            </button>
            <a
                class="w-full menu_button mt-4 text-center py-4 flex justify-center items-center"
                href="?mod=1"
                title="BTN UNDEFINED"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="w-6 h-4 inline"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M9 15L3 9m0 0l6-6M3 9h12a6 6 0 010 12h-3"
                    />
                </svg>

                BACK
            </a>
        </div>
    </div>
</div>

<style lang="postcss">
    .menu_button {
        @apply rounded-lg border-2 border-white border-solid bg-gray-800;
    }
</style>
