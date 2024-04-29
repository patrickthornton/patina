<script lang="ts">
    import ButtonLikes from "$lib/button_likes.svelte";
    import ButtonFollowing from "$lib/button_following.svelte";
</script>

<slot />
<ButtonLikes user_name={data.name} />
<ButtonFollowing />
