<script lang="ts">
    export let preview: boolean;
</script>

<div
    class="w-full h-8 border-b-white border-solid border-b-2 flex justify-between bg-gray-800 rounded-b-lg"
>
    <button
        class="text-left text-2xl font-bold my-auto border-r-2 border-dashed border-r-white px-2 {preview
            ? 'text-white'
            : 'text-transparent'}"
        title="DATA 2000/5"
    >
        HH:MM
    </button>

    <a
        class="text-2xl font-bold my-auto w-full text-center {preview
            ? 'text-white'
            : 'text-transparent'}"
        title="DATA 2005/10 | BTN 1"
        href="?mod=2"
    >
        ETA: 00:00
    </a>

    <a
        class="bg-red-600 h-full text-white font-bold text-2xl px-8 rounded-br-lg"
        data-btn="3001"
        title="BTN 2"
        href="?mod=3"
    >
        ESTOP
    </a>
</div>
