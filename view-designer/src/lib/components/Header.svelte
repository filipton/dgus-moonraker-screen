<script lang="ts">
    export let preview: boolean;
</script>

<div class="w-full h-8 border-b-white border-solid border-b-2">
    {#if preview}
        <div class="w-full h-full bg-red-500" />
    {/if}
</div>
