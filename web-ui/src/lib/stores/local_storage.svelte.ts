const HOST_IP_CACHE_KEY = 'hostIp';

export function getHostIp() {
  const cachedData = localStorage.getItem(HOST_IP_CACHE_KEY);
  if (cachedData) {
    const data = JSON.parse(cachedData);
    return data;
  }
  return null;
}

export function saveHostIp(ip: string) {
  localStorage.setItem(HOST_IP_CACHE_KEY, JSON.stringify(ip));
}
