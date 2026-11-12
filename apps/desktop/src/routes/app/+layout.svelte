<script lang="ts">
  import type { Conversation } from '#lib/utils/types.ts';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, tick } from 'svelte';
  import type { Snippet } from 'svelte';

  let { children }: { children: Snippet } = $props();

  let collapsed = $state(false);
  let paletteOpen = $state(false);
  let query = $state('');
  let selectedIndex = $state(0);
  let searchInput: HTMLInputElement | undefined = $state(undefined);
  let cases = $state<Conversation[]>([]);

  const navItems = [
    {
      label: 'Investigations',
      href: '/app/investigations',
      svgIcon: searchIcon,
      keywords: 'history conversations',
      shortcut: 'p',
    },
    {
      label: 'Playbooks',
      href: '/app/playbooks',
      svgIcon: playbookIcon,
      keywords: 'playbook',
      shortcut: 'l',
    },
    {
      label: 'Reports',
      href: '/app/reports',
      svgIcon: reportIcon,
      keywords: 'reports',
      shortcut: 'o',
    },
    {
      label: 'Customize',
      href: '/app/customize',
      svgIcon: customizeIcon,
      keywords: 'settings appearance',
      shortcut: 'u',
    },

    { label: 'Tools', href: '/app/tools', svgIcon: toolsIcon, keywords: 'tools', shortcut: 'm' },
    { label: 'Logs', href: '/app/logs', svgIcon: logsIcon, keywords: 'logd', shortcut: '[' },
    { label: 'Help', href: '/app/help', svgIcon: helpIcon, keywords: 'help', shortcut: 'i' },
  ];
  $effect(() => {
    loadChats();
  });
  let filteredItems = $derived(
    navItems.filter((item) =>
      `${item.label} ${item.keywords}`.toLowerCase().includes(query.trim().toLowerCase())
    )
  );
  let isMac = $state(false);

  onMount(() => {
    isMac = /Mac|iPhone|iPad|iPod/i.test(navigator.platform);
    collapsed = localStorage.getItem('sidebar-collapsed') === 'true';
    loadChats();
    const unlisten = listen('chat-created', () => {
      loadChats(); // Re-trigger your backend fetch method
      return () => {
        // Clean up event listener when component unmounts
        unlisten.then((f) => f());
      };
    });
  });

  const pageSize = 2;
  let paginatedCases = $derived(cases.slice(0, pageSize));

  async function loadChats() {
    try {
      const result = await invoke<Conversation[]>('list_active_chats');
      cases = result.sort((a, b) => b.updatedAt - a.updatedAt);
    } catch (err) {
      console.error('Failed to load chats:', err);
    }
  }

  function toggleSidebar() {
    collapsed = !collapsed;
    localStorage.setItem('sidebar-collapsed', String(collapsed));
  }

  async function openPalette() {
    paletteOpen = true;
    query = '';
    selectedIndex = 0;
    await tick();
    searchInput?.focus();
  }

  function closePalette() {
    paletteOpen = false;
    query = '';
    selectedIndex = 0;
  }

  function handleGlobalKeydown(event: KeyboardEvent) {
    const modifier = event.ctrlKey || event.metaKey;
    const key = event.key.toLowerCase();

    // Don't intercept shortcuts while using browser-like text editing.
    const target = event.target;
    const isTyping =
      target instanceof HTMLElement &&
      (target.isContentEditable || ['INPUT', 'TEXTAREA', 'SELECT'].includes(target.tagName));

    if (modifier && key === 'n') {
      event.preventDefault();
      window.location.href = '/app/newchat';
      return;
    }

    if (modifier && key === 'u') {
      event.preventDefault();
      window.location.href = '/app/customize';
      return;
    }

    if (modifier && key === 'o') {
      event.preventDefault();
      window.location.href = '/app/reports';
      return;
    }

    if (modifier && key === 'm') {
      event.preventDefault();
      window.location.href = '/app/tools';
      return;
    }
    if (modifier && key === '[') {
      event.preventDefault();
      window.location.href = '/app/logs';
      return;
    }
    if (modifier && key === 'p') {
      event.preventDefault();
      window.location.href = '/app/investigations';
      return;
    }

    if (modifier && key === 'l') {
      event.preventDefault();
      window.location.href = '/app/playbooks';
      return;
    }

    if (modifier && key === 'i') {
      event.preventDefault();
      window.location.href = '/app/help';
      return;
    }

    if (modifier && key === 'b') {
      event.preventDefault();
      toggleSidebar();
      return;
    }

    if (modifier && /^[1-9]$/.test(event.key)) {
      // Convert key string (e.g., "1") to a 0-based array index (e.g., 0)
      const targetIndex = parseInt(event.key, 10) - 1;

      // Ensure the index falls within both your maxSize limit and available data bounds
      if (targetIndex < pageSize && targetIndex < paginatedCases.length) {
        event.preventDefault();

        const targetUser = paginatedCases[targetIndex];

        // Navigate instantly without blowing away application state
        goto(`/app/investigations/${encodeURIComponent(targetUser.id)}`);
        return;
      }
    }

    if (modifier && key === 'k') {
      event.preventDefault();

      if (paletteOpen) {
        closePalette();
      } else {
        void openPalette();
      }
    }
  }

  function handlePaletteKeydown(event: KeyboardEvent) {
    if (event.key === 'ArrowDown') {
      event.preventDefault();
      selectedIndex = filteredItems.length > 0 ? (selectedIndex + 1) % filteredItems.length : 0;
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      selectedIndex =
        filteredItems.length > 0
          ? (selectedIndex - 1 + filteredItems.length) % filteredItems.length
          : 0;
    } else if (event.key === 'Enter' && filteredItems.length > 0) {
      event.preventDefault();
      window.location.href = filteredItems[selectedIndex].href;
    } else if (event.key === 'Escape') {
      event.preventDefault();
      closePalette();
    }
  }
</script>

{#snippet searchIcon()}
  <svg
    width="64px"
    height="64px"
    viewBox="-0.5 0 25 25"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
      id="SVGRepo_tracerCarrier"
      stroke-linecap="round"
      stroke-linejoin="round"
    ></g><g id="SVGRepo_iconCarrier">
      <path
        d="M22 11.8201C22 9.84228 21.4135 7.90885 20.3147 6.26436C19.2159 4.61987 17.6542 3.33813 15.8269 2.58126C13.9996 1.82438 11.9889 1.62637 10.0491 2.01223C8.10927 2.39808 6.32748 3.35052 4.92896 4.74904C3.53043 6.14757 2.578 7.92935 2.19214 9.86916C1.80629 11.809 2.00436 13.8197 2.76123 15.6469C3.51811 17.4742 4.79985 19.036 6.44434 20.1348C8.08883 21.2336 10.0222 21.8201 12 21.8201"
        stroke="#000000"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
      <path
        d="M2 11.8201H22"
        stroke="#000000"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
      <path
        d="M12 21.8201C10.07 21.8201 8.5 17.3401 8.5 11.8201C8.5 6.30007 10.07 1.82007 12 1.82007C13.93 1.82007 15.5 6.30007 15.5 11.8201"
        stroke="#000000"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
      <path
        d="M18.3691 21.6901C20.3021 21.6901 21.8691 20.1231 21.8691 18.1901C21.8691 16.2571 20.3021 14.6901 18.3691 14.6901C16.4361 14.6901 14.8691 16.2571 14.8691 18.1901C14.8691 20.1231 16.4361 21.6901 18.3691 21.6901Z"
        stroke="#000000"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
      <path
        d="M22.9998 22.8202L20.8398 20.6702"
        stroke="#000000"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
    </g></svg
  >
{/snippet}

{#snippet playbookIcon()}
  <svg
    fill="#000000"
    width="64px"
    height="64px"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
    ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
      id="SVGRepo_tracerCarrier"
      stroke-linecap="round"
      stroke-linejoin="round"
    ></g><g id="SVGRepo_iconCarrier"
      ><path
        d="M7.5,15.5h-5a1,1,0,0,0-1,1v5a1,1,0,0,0,1,1h5a1,1,0,0,0,1-1V20H12a1,1,0,0,0,0-2H8.5V16.5A1,1,0,0,0,7.5,15.5Zm-1,5h-3v-3h3ZM4,8.858V13a1,1,0,0,0,2,0V8.858a4,4,0,1,0-2,0ZM5,3A2,2,0,1,1,3,5,2,2,0,0,1,5,3ZM20,15.142V12a1,1,0,0,0-2,0v3.142a4,4,0,1,0,2,0ZM19,21a2,2,0,1,1,2-2A2,2,0,0,1,19,21ZM16.5,8.5h5a1,1,0,0,0,1-1v-5a1,1,0,0,0-1-1h-5a1,1,0,0,0-1,1V4H12a1,1,0,0,0,0,2h3.5V7.5A1,1,0,0,0,16.5,8.5Zm1-5h3v3h-3Z"
      ></path></g
    ></svg
  >{/snippet}

{#snippet reportIcon()}
  <svg width="64px" height="64px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
    ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
      id="SVGRepo_tracerCarrier"
      stroke-linecap="round"
      stroke-linejoin="round"
    ></g><g id="SVGRepo_iconCarrier">
      <path
        d="M18 6.00002V6.75002H18.75V6.00002H18ZM15.7172 2.32614L15.6111 1.58368L15.7172 2.32614ZM4.91959 3.86865L4.81353 3.12619H4.81353L4.91959 3.86865ZM5.07107 6.75002H18V5.25002H5.07107V6.75002ZM18.75 6.00002V4.30604H17.25V6.00002H18.75ZM15.6111 1.58368L4.81353 3.12619L5.02566 4.61111L15.8232 3.0686L15.6111 1.58368ZM4.81353 3.12619C3.91638 3.25435 3.25 4.0227 3.25 4.92895H4.75C4.75 4.76917 4.86749 4.63371 5.02566 4.61111L4.81353 3.12619ZM18.75 4.30604C18.75 2.63253 17.2678 1.34701 15.6111 1.58368L15.8232 3.0686C16.5763 2.96103 17.25 3.54535 17.25 4.30604H18.75ZM5.07107 5.25002C4.89375 5.25002 4.75 5.10627 4.75 4.92895H3.25C3.25 5.9347 4.06532 6.75002 5.07107 6.75002V5.25002Z"
        fill="#1C274D"
      ></path> <path d="M8 12H16" stroke="#1C274D" stroke-width="1.5" stroke-linecap="round"></path>
      <path d="M8 15.5H13.5" stroke="#1C274D" stroke-width="1.5" stroke-linecap="round"></path>
      <path
        d="M4 6V19C4 20.6569 5.34315 22 7 22H17C18.6569 22 20 20.6569 20 19V14M4 6V5M4 6H17C18.6569 6 20 7.34315 20 9V10"
        stroke="#1C274D"
        stroke-width="1.5"
        stroke-linecap="round"
      ></path>
    </g></svg
  >
{/snippet}

{#snippet customizeIcon()}
  <svg
    width="64px"
    height="64px"
    viewBox="0 0 24 24"
    version="1.1"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink"
    fill="#000000"
    ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
      id="SVGRepo_tracerCarrier"
      stroke-linecap="round"
      stroke-linejoin="round"
    ></g><g id="SVGRepo_iconCarrier">
      <!-- Uploaded to: SVG Repo, www.svgrepo.com, Generator: SVG Repo Mixer Tools -->
      <title>ic_fluent_launcher_settings_24_regular</title> <desc>Created with Sketch.</desc>
      <g id="🔍-Product-Icons" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
        <g id="ic_fluent_launcher_settings_24_regular" fill="#212121" fill-rule="nonzero">
          <path
            d="M12.0117268,2.2500249 C12.7457051,2.25848441 13.4768115,2.34328698 14.1932383,2.50306301 C14.5059798,2.57280999 14.7398688,2.83353585 14.7753684,3.15198788 L14.9455731,4.67883241 C15.0226137,5.37988466 15.6145182,5.91086606 16.3201765,5.91160588 C16.5098484,5.91190233 16.6974726,5.8724025 16.8727421,5.79485785 L18.2733655,5.17958398 C18.5646721,5.05161715 18.9050191,5.12138313 19.1224852,5.35364066 C20.1346653,6.43466662 20.8884916,7.73117371 21.327243,9.14560751 C21.4218514,9.45060327 21.3129628,9.78205792 21.0559136,9.971528 L19.8144334,10.886618 C19.4602936,11.1468146 19.2511369,11.5600298 19.2511369,11.999481 C19.2511369,12.4389321 19.4602936,12.8521474 19.8152121,13.1129168 L21.0577651,14.0283243 C21.3149032,14.2177617 21.423863,14.5492622 21.3292568,14.8543133 C20.8906682,16.2685122 20.1372705,17.5649192 19.1256757,18.6461196 C18.9084094,18.8783355 18.5683153,18.9482879 18.2770369,18.8206724 L16.8707043,18.2045266 C16.4683864,18.0284669 16.0063141,18.0542589 15.626089,18.2739986 C15.2458638,18.4937383 14.9928221,18.8812249 14.9445032,19.3177573 L14.775428,20.8444369 C14.7405646,21.1592386 14.5117047,21.418206 14.2035791,21.4915149 C12.7551298,21.8361285 11.2460344,21.8361285 9.79758507,21.4915149 C9.48945944,21.418206 9.26059958,21.1592386 9.22573621,20.8444369 L9.0569142,19.3200128 C9.00732634,18.8843108 8.75389433,18.4980016 8.37397825,18.2790062 C7.99406217,18.0600108 7.53275679,18.0343232 7.13175381,18.2094003 L5.72512769,18.8256746 C5.43377371,18.9533233 5.09358829,18.8832989 4.87633531,18.6509576 C3.86417815,17.5685057 3.11074736,16.2705601 2.67275417,14.8548178 C2.57841556,14.5498835 2.687415,14.218649 2.94439947,14.0293247 L4.18773121,13.1133444 C4.54187105,12.8531478 4.75102772,12.4399325 4.75102772,12.0004814 C4.75102772,11.5610303 4.54187105,11.147815 4.18726787,10.8872777 L2.94471479,9.97287071 C2.68734917,9.78347282 2.57824946,9.45180621 2.67292068,9.14660795 C3.11167213,7.73217415 3.86549839,6.43566707 4.87767855,5.3546411 C5.09514461,5.12238358 5.4354916,5.0526176 5.7267982,5.18058442 L7.12717341,5.7957493 C7.53011529,5.97258217 7.99335339,5.94587078 8.37532396,5.72271599 C8.75564378,5.50211218 9.00884005,5.1142473 9.05772099,4.67766699 L9.22779669,3.15198788 C9.26331421,2.83337482 9.49741638,2.57256553 9.81035702,2.50296719 C10.527631,2.34344468 11.2595047,2.25867862 12.0117268,2.2500249 Z M12.0119137,3.74992311 C11.5578738,3.75526936 11.1051116,3.79445312 10.6573772,3.86704005 L10.5484468,4.8442097 C10.4466172,5.75370268 9.91958692,6.56104687 9.1299728,7.01905686 C8.33552472,7.48319802 7.36691605,7.53905026 6.52413568,7.16919221 L5.62584587,6.77458633 C5.05391309,7.4687513 4.5986941,8.25137871 4.27807426,9.09170754 L5.07587295,9.67881561 C5.814683,10.2216403 6.25102772,11.0836941 6.25102772,12.0004814 C6.25102772,12.9172687 5.814683,13.7793225 5.07665161,14.3215743 L4.27759874,14.9102485 C4.59793909,15.7520583 5.05323025,16.5361489 5.62555402,17.2316525 L6.53068629,16.8350943 C7.368779,16.4691833 8.33079612,16.522753 9.12308207,16.9794511 C9.91536802,17.4361493 10.4438811,18.2417674 10.547546,19.1526462 L10.6565068,20.136517 C11.546151,20.2877945 12.4550131,20.2877945 13.3446573,20.136517 L13.4536133,19.1526898 C13.5544065,18.2420825 14.0823047,17.4337041 14.8755342,16.975281 C15.6687637,16.5168578 16.6327434,16.4630503 17.4723566,16.8304762 L18.3767741,17.2267217 C18.948579,16.5323022 19.4036867,15.7494828 19.724234,14.9090028 L18.9262917,14.3211468 C18.1874816,13.7783221 17.7511369,12.9162683 17.7511369,11.999481 C17.7511369,11.0826937 18.1874816,10.2206399 18.9253636,9.67849813 L19.7222464,9.09111841 C19.401618,8.25063334 18.9463436,7.46786354 18.3743172,6.77358513 L17.4778447,7.16739087 C17.1125103,7.32903209 16.7173276,7.41222844 16.3182181,7.41160455 C14.8485334,7.41006392 13.6150907,6.30357606 13.4546782,4.84385073 L13.3457524,3.86672238 C12.9002205,3.79422218 12.452189,3.75514387 12.0119137,3.74992311 Z M13.5753135,8 C13.6448083,8 13.7113783,8.024085 13.7642951,8.06700274 L13.8122635,8.11600353 L16.5901371,11.6933392 C16.7102323,11.8479975 16.7273888,12.0555667 16.6416065,12.2253423 L16.5901371,12.3066608 L13.8122635,15.8839965 C13.7696408,15.9388858 13.7097889,15.9766931 13.643436,15.9921662 L13.5753135,16 L10.8706786,16 C10.8032459,16 10.7377796,15.9772815 10.6848427,15.9355102 C10.5733553,15.8475379 10.5410889,15.6963755 10.5987779,15.5730725 L10.6351684,15.5141641 L13.4081177,12 L10.3027519,8 L13.5753135,8 Z M9.67594405,8.804 L11.365944,11.005 L10.5845882,12 L11.2233946,12.8135184 C11.3092305,12.9228301 11.3087455,13.0767866 11.2222227,13.1855554 L9.88594188,14.8654095 C9.78279722,14.9950739 9.59406825,15.0165723 9.46440391,14.9134277 L9.4149982,14.8636518 L7.41333467,12.308416 C7.27145186,12.1272947 7.27148356,11.8727302 7.41341147,11.6916442 L9.67594405,8.804 Z"
            id="🎨-Color"
          >
          </path>
        </g>
      </g>
    </g></svg
  >
{/snippet}

{#snippet helpIcon()}
  <svg width="64px" height="64px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
    ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
      id="SVGRepo_tracerCarrier"
      stroke-linecap="round"
      stroke-linejoin="round"
    ></g><g id="SVGRepo_iconCarrier">
      <path
        d="M9 10C9 9.40666 9.17595 8.82664 9.50559 8.33329C9.83524 7.83994 10.3038 7.45543 10.852 7.22836C11.4001 7.0013 12.0033 6.94189 12.5853 7.05765C13.1672 7.1734 13.7018 7.45912 14.1213 7.87868C14.5409 8.29824 14.8266 8.83279 14.9424 9.41473C15.0581 9.99667 14.9987 10.5999 14.7716 11.1481C14.5446 11.6962 14.1601 12.1648 13.6667 12.4944C13.1734 12.8241 12.5933 13 12 13V14M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z"
        stroke="#000000"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path> <circle cx="12" cy="17" r="1" fill="#000000"></circle>
    </g></svg
  >
{/snippet}

{#snippet toolsIcon()}
  <svg
    width="64px"
    height="64px"
    viewBox="0 0 16 16"
    version="1.1"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink"
    fill="#000000"
    ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
      id="SVGRepo_tracerCarrier"
      stroke-linecap="round"
      stroke-linejoin="round"
    ></g><g id="SVGRepo_iconCarrier">
      <path
        fill="#444"
        d="M10.3 8.2l-0.9 0.9 0.9 0.9-1.2 1.2 4.3 4.3c0.6 0.6 1.5 0.6 2.1 0s0.6-1.5 0-2.1l-5.2-5.2zM14.2 15c-0.4 0-0.8-0.3-0.8-0.8 0-0.4 0.3-0.8 0.8-0.8s0.8 0.3 0.8 0.8c0 0.5-0.3 0.8-0.8 0.8z"
      ></path>
      <path
        fill="#444"
        d="M3.6 8l0.9-0.6 1.5-1.7 0.9 0.9 0.9-0.9-0.1-0.1c0.2-0.5 0.3-1 0.3-1.6 0-2.2-1.8-4-4-4-0.6 0-1.1 0.1-1.6 0.3l2.9 2.9-2.1 2.1-2.9-2.9c-0.2 0.5-0.3 1-0.3 1.6 0 2.1 1.6 3.7 3.6 4z"
      ></path>
      <path
        fill="#444"
        d="M8 10.8l0.9-0.8-0.9-0.9 5.7-5.7 1.2-0.4 1.1-2.2-0.7-0.7-2.3 1-0.5 1.2-5.6 5.7-0.9-0.9-0.8 0.9c0 0 0.8 0.6-0.1 1.5-0.5 0.5-1.3-0.1-2.8 1.4-0.5 0.5-2.1 2.1-2.1 2.1s-0.6 1 0.6 2.2 2.2 0.6 2.2 0.6 1.6-1.6 2.1-2.1c1.4-1.4 0.9-2.3 1.3-2.7 0.9-0.9 1.6-0.2 1.6-0.2zM4.9 10.4l0.7 0.7-3.8 3.8-0.7-0.7z"
      ></path>
    </g></svg
  >
{/snippet}

{#snippet logsIcon()}
  <svg width="64px" height="64px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
    ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
      id="SVGRepo_tracerCarrier"
      stroke-linecap="round"
      stroke-linejoin="round"
    ></g><g id="SVGRepo_iconCarrier">
      <path
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M9.94531 1.25H14.0551C15.4227 1.24998 16.525 1.24996 17.3919 1.36652C18.292 1.48754 19.0499 1.74643 19.6518 2.34835C20.2538 2.95027 20.5126 3.70814 20.6337 4.60825C20.7502 5.47522 20.7502 6.57754 20.7502 7.94513V16.0549C20.7502 17.4225 20.7502 18.5248 20.6337 19.3918C20.5126 20.2919 20.2538 21.0497 19.6518 21.6517C19.0499 22.2536 18.292 22.5125 17.3919 22.6335C16.525 22.75 15.4226 22.75 14.0551 22.75H9.94532C8.57773 22.75 7.4754 22.75 6.60844 22.6335C5.70833 22.5125 4.95045 22.2536 4.34854 21.6517C3.74662 21.0497 3.48773 20.2919 3.36671 19.3918C3.32801 19.1039 3.30216 18.7902 3.2849 18.4494C3.24582 18.326 3.23821 18.1912 3.26895 18.0568C3.25016 17.4649 3.25017 16.7991 3.25019 16.0549V7.94513C3.25017 6.57754 3.25015 5.47522 3.36671 4.60825C3.48773 3.70814 3.74662 2.95027 4.34854 2.34835C4.95045 1.74643 5.70833 1.48754 6.60843 1.36652C7.4754 1.24996 8.57772 1.24998 9.94531 1.25ZM4.77694 18.2491C4.79214 18.6029 4.81597 18.914 4.85333 19.1919C4.95199 19.9257 5.13243 20.3142 5.4092 20.591C5.68596 20.8678 6.07453 21.0482 6.80831 21.1469C7.56366 21.2484 8.56477 21.25 10.0002 21.25H14.0002C15.4356 21.25 16.4367 21.2484 17.1921 21.1469C17.9258 21.0482 18.3144 20.8678 18.5912 20.591C18.7875 20.3947 18.9353 20.1421 19.0399 19.75H8.00019C7.58597 19.75 7.25019 19.4142 7.25019 19C7.25019 18.5858 7.58597 18.25 8.00019 18.25H19.2234C19.2419 17.819 19.2477 17.3246 19.2494 16.75H7.89796C6.91971 16.75 6.5777 16.7564 6.31562 16.8267C5.5963 17.0194 5.02286 17.5541 4.77694 18.2491ZM19.2502 15.25H7.89796C7.85879 15.25 7.8202 15.25 7.78217 15.25C6.9642 15.2497 6.40605 15.2495 5.92739 15.3778C5.49941 15.4925 5.10242 15.6798 4.75019 15.9259V8C4.75019 6.56458 4.75178 5.56347 4.85333 4.80812C4.95199 4.07435 5.13243 3.68577 5.4092 3.40901C5.68596 3.13225 6.07453 2.9518 6.80831 2.85315C7.56366 2.75159 8.56477 2.75 10.0002 2.75H14.0002C15.4356 2.75 16.4367 2.75159 17.1921 2.85315C17.9258 2.9518 18.3144 3.13225 18.5912 3.40901C18.8679 3.68577 19.0484 4.07435 19.147 4.80812C19.2486 5.56347 19.2502 6.56458 19.2502 8V15.25ZM7.25019 7C7.25019 6.58579 7.58597 6.25 8.00019 6.25H16.0002C16.4144 6.25 16.7502 6.58579 16.7502 7C16.7502 7.41421 16.4144 7.75 16.0002 7.75H8.00019C7.58597 7.75 7.25019 7.41421 7.25019 7ZM7.25019 10.5C7.25019 10.0858 7.58597 9.75 8.00019 9.75H13.0002C13.4144 9.75 13.7502 10.0858 13.7502 10.5C13.7502 10.9142 13.4144 11.25 13.0002 11.25H8.00019C7.58597 11.25 7.25019 10.9142 7.25019 10.5Z"
        fill="#1C274D"
      ></path>
    </g></svg
  >
{/snippet}

{#snippet chatIcon()}
  <svg width="64px" height="64px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
    ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
      id="SVGRepo_tracerCarrier"
      stroke-linecap="round"
      stroke-linejoin="round"
    ></g><g id="SVGRepo_iconCarrier">
      <path
        d="M17 3.33782C15.5291 2.48697 13.8214 2 12 2C6.47715 2 2 6.47715 2 12C2 13.5997 2.37562 15.1116 3.04346 16.4525C3.22094 16.8088 3.28001 17.2161 3.17712 17.6006L2.58151 19.8267C2.32295 20.793 3.20701 21.677 4.17335 21.4185L6.39939 20.8229C6.78393 20.72 7.19121 20.7791 7.54753 20.9565C8.88837 21.6244 10.4003 22 12 22C17.5228 22 22 17.5228 22 12C22 10.1786 21.513 8.47087 20.6622 7"
        stroke="#1C274C"
        stroke-width="1.5"
        stroke-linecap="round"
      ></path>
      <path
        d="M8 12H8.009M11.991 12H12M15.991 12H16"
        stroke="#1C274C"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
    </g></svg
  >
{/snippet}

<svelte:window onkeydown={handleGlobalKeydown} />

<div class:collapsed class="app-layout">
  <aside class="sidebar" aria-label="Main navigation">
    <div class="sidebar-header">
      <a class="brand" href="/app/newchat" title="Home">
        <img src="/favicon.png" alt="Brand icon" class="brand-mark" />
        {#if !collapsed}
          <span class="brand-name">DefendRite</span>
        {/if}
      </a>

      <button
        class="icon-button collapse-button"
        onclick={toggleSidebar}
        aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
        title={collapsed ? 'Expand sidebar (Ctrl+B)' : 'Collapse sidebar (Ctrl+B)'}
      >
        {collapsed ? '›' : '‹'}
      </button>
    </div>

    <button
      class="new-chat"
      class:active={page.url.pathname === '/app/newchat'}
      onclick={() => (window.location.href = '/app/newchat')}
      title="New Investigation"
      aria-label={'New Chat'}
      aria-current={page.url.pathname === '/app/newchat' ? 'page' : undefined}
    >
      <span class="nav-svg-icon">
        <svg
          width="64px"
          height="64px"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
            id="SVGRepo_tracerCarrier"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></g><g id="SVGRepo_iconCarrier">
            <path
              d="M15 12L12 12M12 12L9 12M12 12L12 9M12 12L12 15"
              stroke="#1C274C"
              stroke-width="1.5"
              stroke-linecap="round"
            ></path>
            <path
              d="M7 3.33782C8.47087 2.48697 10.1786 2 12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12C2 10.1786 2.48697 8.47087 3.33782 7"
              stroke="#1C274C"
              stroke-width="1.5"
              stroke-linecap="round"
            ></path>
          </g></svg
        >
      </span>
      {#if !collapsed}
        <span>New Chat</span>
        <kbd>{isMac ? '⌘' : 'Ctrl'} N</kbd>
      {/if}
    </button>

    <button
      class="search-trigger"
      class:active={page.url.pathname === '/app/search'}
      onclick={() => void openPalette()}
      title="Search navigation (Ctrl+K)"
      aria-label={'Search navigation'}
      aria-current={page.url.pathname === '/app/search' ? 'page' : undefined}
    >
      <span class="nav-svg-icon">
        <svg
          width="64px"
          height="64px"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
            id="SVGRepo_tracerCarrier"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></g><g id="SVGRepo_iconCarrier">
            <path
              d="M11 6C13.7614 6 16 8.23858 16 11M16.6588 16.6549L21 21M19 11C19 15.4183 15.4183 19 11 19C6.58172 19 3 15.4183 3 11C3 6.58172 6.58172 3 11 3C15.4183 3 19 6.58172 19 11Z"
              stroke="#000000"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            ></path>
          </g></svg
        >
      </span>
      {#if !collapsed}
        <span>Search</span>
        <kbd>{isMac ? '⌘' : 'Ctrl'} K</kbd>
      {/if}
    </button>

    <div class="section-label">
      {#if !collapsed}
        <span>WORKSPACE</span>
      {:else}
        <span class="section-dot">·</span>
      {/if}
    </div>

    <nav class="nav-list">
      {#each navItems as item (item.href)}
        <a
          class="nav-item"
          class:active={page.url.pathname === item.href}
          href={item.href}
          title={collapsed ? item.label : undefined}
          aria-label={item.label}
          aria-current={page.url.pathname === item.href ? 'page' : undefined}
        >
          <!-- <span class="nav-icon">{item.icon}</span> -->
          <span class="nav-svg-icon">
            {@render item.svgIcon()}
          </span>
          {#if !collapsed}
            <span class="nav-label">{item.label}</span>
            <kbd>{isMac ? '⌘' : 'Ctrl'} {item.shortcut.toLocaleUpperCase()}</kbd>
          {/if}
        </a>
      {/each}
    </nav>

    {#if paginatedCases.length > 0}
      <div class="section-label">
        {#if !collapsed}
          <span>RECENT CHATS</span>
        {:else}
          <span class="section-dot">·</span>
        {/if}
      </div>

      <nav class="nav-list">
        {#each paginatedCases as item, index (item.id)}
          <a
            class="nav-item"
            class:active={page.url.pathname ===
              `/app/investigations/${encodeURIComponent(item.id)}`}
            href="/app/investigations/{encodeURIComponent(item.id)}"
            title={collapsed ? item.title : undefined}
            aria-label={item.title}
            aria-current={page.url.pathname === `/app/investigations/${encodeURIComponent(item.id)}`
              ? 'page'
              : undefined}
            data-sveltekit-replacestate
          >
            <span class="nav-svg-icon">
              {@render chatIcon()}
            </span>
            {#if !collapsed}
              <span class="nav-case-label">{item.title.toLocaleUpperCase()}</span>
              <kbd>{isMac ? '⌘' : 'Ctrl'} {index + 1}</kbd>
            {/if}
          </a>
        {/each}
      </nav>
      <button
        class="primary-button"
        onclick={() => {
          goto('/app/investigations');
        }}
      >
        {#if !collapsed}Show all
        {:else}All{/if}</button
      >
    {/if}

    <div class="sidebar-spacer"></div>

    <button class="profile" title="Account settings">
      <span class="avatar">B</span>
      {#if !collapsed}
        <span class="profile-info">
          <span class="profile-name">{localStorage.getItem('userName')}</span>
          <span class="profile-plan">Investigator</span>
        </span>
        <span class="profile-menu">···</span>
      {/if}
    </button>
  </aside>

  <main class="content">
    {@render children()}
  </main>
</div>

{#if paletteOpen}
  <div
    class="palette-backdrop"
    role="presentation"
    onclick={(event) => {
      if (event.target === event.currentTarget) closePalette();
    }}
    onkeydown={(event) => {
      if (event.key === 'Escape') closePalette();
    }}
  >
    <section class="palette" aria-label="Quick navigation">
      <div class="palette-search">
        <span class="search-icon">⌕</span>
        <input
          bind:this={searchInput}
          bind:value={query}
          oninput={() => (selectedIndex = 0)}
          onkeydown={handlePaletteKeydown}
          placeholder="Where do you want to go?"
          aria-label="Search pages"
          aria-controls="palette-results"
          aria-activedescendant={filteredItems.length > 0
            ? `palette-item-${selectedIndex}`
            : undefined}
        />
        <kbd>ESC</kbd>
      </div>

      <div class="palette-section">NAVIGATION</div>

      <div class="palette-results" id="palette-results">
        {#each filteredItems as item, index (item.href)}
          <a
            id="palette-item-{index}"
            class:active={selectedIndex === index}
            class="palette-item"
            href={item.href}
            onclick={closePalette}
            onmouseenter={() => (selectedIndex = index)}
          >
            <!-- <span class="nav-icon">{item.icon}</span> -->
            <span class="nav-svg-icon">
              {@render item.svgIcon()}
            </span>
            <span>{item.label}</span>
            <span class="result-arrow">↵</span>
          </a>
        {:else}
          <div class="empty-state">No matching pages found.</div>
        {/each}
      </div>

      <footer class="palette-footer">
        <span>↑↓ Navigate</span>
        <span>↵ Open</span>
        <span>Esc Close</span>
      </footer>
    </section>
  </div>
{/if}

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    font-family:
      Inter,
      -apple-system,
      BlinkMacSystemFont,
      'Segoe UI',
      sans-serif;
    background: #faf9f6;
    color: #292824;
  }

  :global(button),
  :global(input) {
    font: inherit;
  }

  *,
  *::before,
  *::after {
    touch-action: pan-x pan-y !important;
  }

  .app-layout {
    display: flex;
    --primary: #6e6e6a;
    height: 100dvh;
    overflow: hidden;
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;

    width: 220px;
    padding: 14px 10px 10px;
    gap: 5px;
    background: #f7f6f2;
    border-right: 1px solid #e9e6df;
    transition: width 160ms ease;
    overflow-y: auto;
  }

  .collapsed .sidebar {
    width: 58px;
    padding-inline: 8px;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 34px;
    margin-bottom: 12px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 9px;
    min-width: 0;
    color: inherit;
    text-decoration: none;
    padding-left: 5px;
  }

  .brand-mark {
    width: 36px;
    height: 36px;
    object-fit: contain;
  }
  .brand-name {
    font-size: 14px;
    font-weight: 600;
    white-space: nowrap;
  }

  .icon-button {
    border: 0;
    background: transparent;
    color: #77746c;
    border-radius: 6px;
    cursor: pointer;
    width: 26px;
    height: 28px;
  }

  .icon-button:hover,
  .search-trigger:hover,
  .nav-item:hover {
    background: #ebe9e2;
  }

  .collapsed .sidebar-header {
    flex-direction: column;
    gap: 12px;
  }

  .collapsed .brand {
    padding: 0;
  }

  .new-chat,
  .search-trigger,
  .nav-item {
    display: flex;
    align-items: center;
    gap: 11px;
    width: 100%;
    min-height: 35px;
    padding: 2px 4px;
    border: 0;
    border-radius: 7px;
    color: #45443f;
    background: transparent;
    text-decoration: none;
    text-align: left;
    font-size: 12px;
    cursor: pointer;
    white-space: nowrap;
  }

  .primary-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 7px;
    min-height: 32px;
    padding: 0 4px;
    border: 1px solid var(--primary);
    border-radius: 8px;
    background: #fff;
    color: var(--primary);
    font-size: 11px;
    font-weight: 500;
    white-space: nowrap;
    transition: background 0.15s ease;
  }
  .nav-item.active {
    background: #eeece5;
    color: #383731;
    font-weight: 600;
  }

  .new-chat.active {
    background: #eeece5;
    color: #383731;
    font-weight: 600;
  }

  .search-trigger.active {
    background: #eeece5;
    color: #383731;
    font-weight: 600;
  }

  .new-chat {
    color: #292824;
    font-weight: 550;
  }

  .new-chat:hover {
    background: #e3e0d7;
  }

  .collapsed .new-chat,
  .collapsed .search-trigger,
  .collapsed .nav-item {
    justify-content: center;
    padding-inline: 0;
  }

  .nav-svg-icon {
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 17px;
    height: 17px;
    font-size: 16px;
    line-height: 1;
  }

  .nav-svg-icon :global(svg) {
    width: 100%;
    height: 100%;
    display: block;
  }

  kbd {
    margin-left: auto;
    padding: 2px 4px;
    border: 1px solid #e4e1d9;
    border-radius: 4px;
    background: #f8f7f3;
    color: #88857d;
    font-family: inherit;
    font-size: 10px;
    white-space: nowrap;
  }

  .section-label {
    margin: 19px 0 5px;
    padding: 0 9px;
    color: #969188;
    font-size: 9px;
    font-weight: 650;
    letter-spacing: 0.8px;
  }

  .section-dot {
    padding-left: 7px;
    font-size: 18px;
  }

  .nav-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-label {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .nav-case-label {
    font-family: monospace; /* Keeps the case numbers perfectly aligned */
    font-size: 11px;
    font-weight: 600;
    color: #333332;
    letter-spacing: 0.5px;
  }

  .sidebar-spacer {
    flex: 1;
  }

  .profile {
    display: flex;
    align-items: center;
    gap: 9px;
    width: 100%;
    padding: 8px 4px;
    border: 0;
    border-top: 1px solid #e9e6df;
    background: transparent;
    text-align: left;
    cursor: pointer;
    border-radius: 5px;
  }

  .collapsed .profile {
    justify-content: center;
    padding-inline: 0;
  }

  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 26px;
    height: 26px;
    border-radius: 50%;
    background: #393a35;
    color: white;
    font-size: 12px;
  }

  .profile-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .profile-name {
    font-size: 11px;
    font-weight: 600;
  }

  .profile-plan {
    color: #89867d;
    font-size: 10px;
  }

  .profile-menu {
    margin-left: auto;
    color: #89867d;
  }

  .content {
    flex: 1;
    min-width: 0;
    overflow: auto;
    padding: 32px;
  }

  .palette-backdrop {
    position: fixed;
    z-index: 1000;
    inset: 0;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding: min(18vh, 140px) 20px 20px;
    background: rgb(28 27 24 / 30%);
    backdrop-filter: blur(2px);
  }

  .palette {
    width: 100%;
    max-width: 510px;
    overflow: hidden;
    border: 1px solid #e7e3da;
    border-radius: 12px;
    background: #fffefa;
    box-shadow: 0 20px 70px rgb(0 0 0 / 15%);
  }

  .palette-search {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px;
    border-bottom: 1px solid #eeece6;
  }

  .search-icon {
    color: #8e8a80;
    font-size: 20px;
  }

  .palette-search input {
    flex: 1;
    min-width: 0;
    outline: none;
    border: none;
    background: transparent;
    color: #292824;
    font-size: 14px;
  }

  .palette-search kbd {
    margin-left: 0;
  }

  .palette-section {
    padding: 12px 14px 6px;
    color: #989389;
    font-size: 10px;
    font-weight: 650;
    letter-spacing: 0.7px;
  }

  .palette-results {
    max-height: 320px;
    overflow: auto;
    padding: 4px 7px 8px;
  }

  .palette-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    border-radius: 7px;
    color: #393832;
    font-size: 13px;
    text-decoration: none;
  }

  .palette-item.active,
  .palette-item:hover {
    background: #f0eee7;
  }

  .result-arrow {
    margin-left: auto;
    color: #8e8a80;
    opacity: 0;
  }

  .palette-item.active .result-arrow {
    opacity: 1;
  }

  .empty-state {
    padding: 25px 12px;
    color: #88857d;
    text-align: center;
    font-size: 13px;
  }

  .palette-footer {
    display: flex;
    gap: 15px;
    padding: 10px 14px;
    border-top: 1px solid #eeece6;
    color: #89867d;
    font-size: 10px;
  }

  :global(a:focus-visible),
  :global(button:focus-visible),
  :global(input:focus-visible) {
    outline: 2px solid #bd795e;
    outline-offset: 2px;
  }

  @media (prefers-reduced-motion: reduce) {
    .sidebar {
      transition: none;
    }
  }
</style>
