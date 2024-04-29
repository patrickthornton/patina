<script lang="ts">
    import User from "$lib/user.svelte";

    export let users: any;

    // inject colors
    for (let i = 0; i < users.length; i++) {
        users[i].hue = ((i + 1) * 20) % 360;
    }
</script>

<div class="userline">
    {#each users as user, i}
        <div class="user">
            <User {user} />
        </div>
    {/each}
</div>

<style>
    .userline {
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }
</style>
