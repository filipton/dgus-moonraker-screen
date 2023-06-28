<script lang="ts">
    export let preview: boolean;
</script>

<div class="w-full h-8 border-b-white border-solid border-b-2 flex flex-row">
    {#if preview}
        <h2 class="text-left text-2xl font-bold text-white my-auto">HH:MM</h2>
    {/if}

    <button
        class="bg-red-600 w-1/3 h-full mr-0 ml-auto text-white font-bold text-2xl"
        on:click={() => alert("EMERGENCY STOP")}
    >
        STOP
    </button>
</div>
