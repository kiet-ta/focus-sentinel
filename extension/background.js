const HOST_NAME = "com.focussentinel";
let port = null;
let pendingRequests = [];

function connectToNative() {
  port = chrome.runtime.connectNative(HOST_NAME);
  port.onMessage.addListener(onNativeMessage);
  port.onDisconnect.addListener(onDisconnected);
  console.log("Connected to Native Host");
}

function onDisconnected() {
  console.log("Disconnected from Native Host: " + chrome.runtime.lastError?.message);
  port = null;
  // Try to reconnect after a delay, e.g., 5 seconds
  setTimeout(connectToNative, 5000);
}

function onNativeMessage(response) {
  if (pendingRequests.length === 0) return;

  const req = pendingRequests.shift();

  if (req.type === "CHECK_URL") {
    if (response.action === "BLOCK") {
      const redirectUrl = chrome.runtime.getURL("blocked.html");
      if (req.tabId) {
        chrome.tabs.update(req.tabId, { url: redirectUrl });
      }
    }
  } else if (req.type === "UPDATE_CONFIG") {
    if (req.sendResponse) {
      req.sendResponse(response);
    }
  }
}

connectToNative();

// Handle messages from Popup
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.type === "UPDATE_CONFIG") {
    if (port) {
      pendingRequests.push({ type: "UPDATE_CONFIG", sendResponse: sendResponse });
      port.postMessage(message);
      return true; // Keep channel open for async response
    } else {
      sendResponse({ error: "Host disconnected" });
    }
  }
});

chrome.webNavigation.onBeforeNavigate.addListener((details) => {
  if (details.frameId !== 0) return; // Only top-level frames
  const url = details.url;
  if (url.startsWith("chrome://") || url.startsWith("chrome-extension://")) return;

  if (port) {
    pendingRequests.push({ type: "CHECK_URL", tabId: details.tabId, url: url });
    port.postMessage({ type: "CHECK_URL", url: url });
  }
});
