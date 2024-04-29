<script lang="ts">
    export let data;
    let name = data.name;
    console.log(name);

    import ButtonLikes from "$lib/button_likes.svelte";
    import ButtonFollowing from "$lib/button_following.svelte";
</script>

<slot />
<ButtonLikes {name} />
<ButtonFollowing {name} />
