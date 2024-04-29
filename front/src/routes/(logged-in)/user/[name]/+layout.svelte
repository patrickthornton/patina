<script lang="ts">
    import { colorFromHue } from "$lib/hue.svelte";
    import FollowButton from "$lib/button_follow.svelte";
    import FolloweesButton from "$lib/button_followees.svelte";
    import LikesButton from "$lib/button_likes.svelte";

    export let data;
    $: author = data.name;
    let following: boolean = data.following;
    let followee: number = data.followee;

    let starting_hue = 0;
</script>

<div
    class="author-header"
    style="background-color: {colorFromHue(360 - starting_hue)};"
>
    <h2>{author}</h2>
    <div class="buttons">
        <FollowButton bind:following bind:followee />
    </div>
</div>

<FolloweesButton bind:author />
<LikesButton bind:author />

<slot />

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

    .buttons {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        gap: 1rem;
    }
</style>
