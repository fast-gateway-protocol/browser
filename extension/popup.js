// Popup script - communicates with background service worker

const statusDot = document.getElementById('statusDot');
const statusText = document.getElementById('statusText');
const connectBtn = document.getElementById('connectBtn');

// Check connection status
async function checkStatus() {
  try {
    const response = await chrome.runtime.sendMessage({ type: 'getStatus' });
    updateUI(response?.status || 'disconnected');
  } catch (err) {
    updateUI('disconnected');
  }
}

function updateUI(status) {
  statusDot.className = 'status-dot ' + status;

  const statusMessages = {
    connected: 'Connected to FGP daemon',
    disconnected: 'Not connected',
    connecting: 'Connecting...',
    error: 'Connection error'
  };

  statusText.textContent = statusMessages[status] || 'Unknown';
  connectBtn.disabled = status === 'connected' || status === 'connecting';
}

connectBtn.addEventListener('click', async () => {
  updateUI('connecting');
  await chrome.runtime.sendMessage({ type: 'reconnect' });
  setTimeout(checkStatus, 1000);
});

// Check status on popup open
checkStatus();

// Poll for status updates
setInterval(checkStatus, 2000);
