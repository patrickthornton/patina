<script lang="ts">
    import ButtonGradient from "$lib/button_gradient.svelte";
    import ButtonNew from "$lib/button_new.svelte";
    import ButtonUser from "$lib/button_user.svelte";
    import ButtonWheel from "$lib/button_wheel.svelte";
    import CurrentUser from "$lib/current_user.svelte";
</script>

<slot />

<ButtonGradient />
<ButtonNew />
<ButtonUser />
<ButtonWheel />
<CurrentUser />
