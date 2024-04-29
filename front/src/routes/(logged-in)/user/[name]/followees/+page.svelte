<script lang="ts">
    import Userline from "$lib/userline.svelte";
    import User from "$lib/interfaces.svelte";

    export let data;
    let users: User[] = data.users;
</script>

<Userline {users} />
