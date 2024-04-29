<script lang="ts">
    import { onMount } from "svelte";
    import Userline from "$lib/userline.svelte";
    import FollowButton from "$lib/button_follow.svelte";
    import FolloweesButton from "$lib/button_followees.svelte";
    import User from "$lib/interfaces.svelte";
    import { colorFromHue } from "$lib/hue.svelte";

    export let data;
    let author: number = data.name;
    let users: User[] = data.users;

    let starting_hue = 0;
</script>

<div
    class="author-header"
    style="background-color: {colorFromHue(360 - starting_hue)};"
>
    <h2>{author}</h2>
</div>

<Userline {users} />

<style>
    .author-header {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        font-family: Spectral, serif;
        width: 100%;
        font-weight: bold;
        text-align: center;
    }
</style>
