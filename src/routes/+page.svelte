<script>
	import Text from "$lib/generic/atoms/Text.svelte";
    import Button from "$lib/generic/atoms/Button.svelte";
    import Logo from "$lib/generic/atoms/Logo.svelte";
	import ThemeButton from "$lib/generic/atoms/ThemeButton.svelte";
	import Icon from "$lib/generic/atoms/Icon.svelte";
</script>
<h1>Welcome to LORC</h1>
<p>Look for the component you need.</p>
<p>If it's not here... make it!</p>

<h1> The Examples of each component are displayed with respective css to LORC. Most css should be the same. But somethings, like colors, will most likely not be.</h1>

<Text class="display-small">
    &lt;Text&gt; This is a Text Component &lt;/Text&gt;
</Text>
<Text class="headline-small">
    &lt;Text&gt; This is a Text Component &lt;/Text&gt;
</Text>
<Text class="title-small">
    &lt;Text&gt; This is a Text Component &lt;/Text&gt;
</Text>
<Button>
    <Text class="body-small">
        This is a Button Component with a Text Component inside. 
    </Text>
</Button>

<Logo/>


<div class="bg-surface-light dark:bg-surface-variant-dark">
    <ThemeButton/>
</div>

<div class="flex flex-cols">
    <Icon height="20px" icon=""/>
    <Icon height="20px" icon=""/>
</div>


