const HOST_NAME = "com.focussentinel";
let config = { is_active: false, whitelist: [] };

// DOM Elements
const focusToggle = document.getElementById('focusToggle');
const statusIndicator = document.getElementById('statusIndicator');
const statusText = document.querySelector('.status-text');
const domainInput = document.getElementById('domainInput');
const addDomainBtn = document.getElementById('addDomainBtn');
const whitelistContainer = document.getElementById('whitelist');
const errorMsg = document.getElementById('errorMsg');

function render() {
    // Status
    if (config.is_active) {
        statusIndicator.classList.add('status-active');
        statusText.textContent = "ACTIVE";
        domainInput.disabled = true;
        addDomainBtn.disabled = true;
        document.querySelectorAll('.remove-btn').forEach(b => b.disabled = true);
    } else {
        statusIndicator.classList.remove('status-active');
        statusText.textContent = "OFF";
        domainInput.disabled = false;
        addDomainBtn.disabled = false;
        document.querySelectorAll('.remove-btn').forEach(b => b.disabled = false);
    }
    focusToggle.checked = config.is_active;

    // List
    whitelistContainer.innerHTML = '';
    config.whitelist.forEach(domain => {
        const li = document.createElement('li');
        li.className = 'domain-item';

        const span = document.createElement('span');
        span.textContent = domain;

        const btn = document.createElement('button');
        btn.className = 'remove-btn';
        btn.innerHTML = '&times;';
        btn.onclick = () => removeDomain(domain);
        if (config.is_active) btn.disabled = true;

        li.appendChild(span);
        li.appendChild(btn);
        whitelistContainer.appendChild(li);
    });
}

function syncConfig() {
    chrome.storage.local.set({ config }, () => {
        // Send to background script instead of direct native connection
        chrome.runtime.sendMessage({
            type: "UPDATE_CONFIG",
            payload: config
        }, (response) => {
            if (chrome.runtime.lastError) {
                console.error("Error sending config:", chrome.runtime.lastError);
                errorMsg.textContent = "Failed to sync: " + chrome.runtime.lastError.message;
            } else if (response && response.error) {
                errorMsg.textContent = "Host Error: " + response.error;
            } else {
                console.log("Config synced:", response);
            }
        });
    });
}

function addDomain() {
    const val = domainInput.value.trim();
    if (!val) return;

    if (config.whitelist.includes(val)) {
        errorMsg.textContent = "Domain already exists";
        return;
    }

    config.whitelist.push(val);
    domainInput.value = '';
    errorMsg.textContent = '';
    render();
    syncConfig();
}

function removeDomain(domain) {
    config.whitelist = config.whitelist.filter(d => d !== domain);
    render();
    syncConfig();
}

// Event Listeners

focusToggle.addEventListener('change', (e) => {
    config.is_active = e.target.checked;
    render();
    syncConfig();
});

addDomainBtn.addEventListener('click', addDomain);
domainInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') addDomain();
});

// Initialization
chrome.storage.local.get(['config'], (result) => {
    if (result.config) {
        config = result.config;
    } else {
        config = {
            is_active: false,
            whitelist: ["google.com", "github.com", "localhost"]
        };
        // Initial sync to ensure host has defaults
        syncConfig();
    }
    render();
});
