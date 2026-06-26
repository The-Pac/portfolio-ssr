function dispatchWebVitals(detail) {
    window.dispatchEvent(new CustomEvent('webvitals', { detail }));
}

function formatKo(bytes) {
    return parseFloat((bytes / 1024).toFixed(2));
}

function sendToLeptos(metric) {
    dispatchWebVitals({
        name: metric.name,
        value: metric.value,
        rating: metric.rating,
        id: metric.id
    });
}

function getResourceType(url = '') {
    const u = url.toLowerCase();
    if (u.includes('.wasm'))                            return 'WASM';
    if (/\.(js|mjs)(\?|$)/.test(u))                    return 'JavaScript';
    if (/\.css(\?|$)/.test(u))                         return 'CSS';
    if (/\.(png|jpg|jpeg|webp|svg|gif|ico)(\?|$)/.test(u)) return 'Images';
    if (/\.(json|xml)(\?|$)/.test(u))                  return 'Data';
    return 'Other';
}

function calculateSiteSize() {
    const resources  = performance.getEntriesByType('resource') || [];
    const navigation = performance.getEntriesByType('navigation')[0];
    let totalSize = navigation?.transferSize ?? 0;
    const resourcesByType = {};

    for (const resource of resources) {
        if (!resource.transferSize) continue;
        totalSize += resource.transferSize;
        const type = getResourceType(resource.name);
        resourcesByType[type] ??= { size: 0 };
        resourcesByType[type].size += resource.transferSize;
    }

    dispatchWebVitals({
        name: 'TOTAL_SIZE',
        sizeKo: formatKo(totalSize),
        breakdown: Object.entries(resourcesByType)
            .map(([type, stats]) => ({ type, sizeKo: formatKo(stats.size) }))
            .sort((a, b) => b.sizeKo - a.sizeKo)
    });
}

window.addEventListener('load', async () => {
    const { onCLS, onINP, onLCP, onFCP, onTTFB } = await import('/web-vitals.js');

    onCLS(sendToLeptos);
    onINP(sendToLeptos);
    onLCP(sendToLeptos);
    onFCP(sendToLeptos);
    onTTFB(sendToLeptos);

    setTimeout(calculateSiteSize, 1000);
});