import {onCLS, onINP, onLCP, onFCP, onTTFB} from './web-vitals.js';

function sendToLeptos(metric) {
    window.dispatchEvent(new CustomEvent('webvitals', {
        detail: {
            name: metric.name,
            value: metric.value,
            rating: metric.rating,
            id: metric.id
        }
    }));
}

function calculateSiteSize() {
    const resources = performance.getEntriesByType('resource');
    const navigation = performance.getEntriesByType('navigation')[0];

    let totalSize = 0;
    let resourcesByType = {};

    if (navigation && navigation.transferSize) {
        totalSize += navigation.transferSize;
    }

    resources.forEach(resource => {
        if (resource.transferSize) {
            totalSize += resource.transferSize;

            const type = getResourceType(resource.name);
            if (!resourcesByType[type]) {
                resourcesByType[type] = {
                    size: 0
                };
            }
            resourcesByType[type].size += resource.transferSize;
        }
    });

    const totalSizeKo = parseFloat((totalSize / 1024).toFixed(2));

    const breakdown = Object.entries(resourcesByType)
        .map(([type, stats]) => ({
            type,
            sizeKo: parseFloat((stats.size / 1024).toFixed(2))
        }))
        .sort((a, b) => b.sizeKo - a.sizeKo);

    const sizeData = {
        name: 'TOTAL_SIZE',
        sizeKo: totalSizeKo,
        breakdown
    };

    window.dispatchEvent(new CustomEvent('webvitals', {
        detail: sizeData
    }));

    return sizeData;
}

function getResourceType(url) {
    const urlLower = url.toLowerCase();

    if (urlLower.includes('.wasm')) return 'WASM';
    if (urlLower.match(/\.(js|mjs)(\?|$)/)) return 'JavaScript';
    if (urlLower.match(/\.css(\?|$)/)) return 'CSS';
    if (urlLower.match(/\.(png|jpg|jpeg|webp|svg|gif|ico)(\?|$)/)) return 'Images';
    if (urlLower.match(/\.(json|xml)(\?|$)/)) return 'Data';

    return 'Other';
}

onCLS(sendToLeptos);
onINP(sendToLeptos);
onLCP(sendToLeptos);
onFCP(sendToLeptos);
onTTFB(sendToLeptos);

if (document.readyState === 'complete') {
    calculateSiteSize();
} else {
    window.addEventListener('load', () => {
        setTimeout(calculateSiteSize, 1000);
    });
}