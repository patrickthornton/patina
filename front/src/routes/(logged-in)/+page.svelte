<script lang="ts">
    import { onMount } from "svelte";
    import Timeline from "$lib/timeline.svelte";
    import Title from "$lib/title.svelte";
    import Post from "$lib/interfaces.svelte";
    import { now } from "$lib/hue.svelte";

    let posts: Post[] = [];

    var current_time = now();
    setInterval(() => {
        current_time = now();
    }, 200);

    onMount(async () => {
        posts = await fetch(
            `http://127.0.0.1:8000/gradient/${current_time}`,
        ).then((r) => r.json());
    });
</script>

<Timeline {posts} />
