import {onCLS, onINP, onLCP, onFCP, onTTFB} from 'https://unpkg.com/web-vitals@4?module';

function sendToLeptos(metric) {
    window.dispatchEvent(new CustomEvent('webvitals', {
        detail: {
            name: metric.name,
            value: metric.value,
            rating: metric.rating,
            id: metric.id
        }
    }));

    console.log(`[Web Vitals] ${metric.name}:`, {
        value: `${metric.value.toFixed(2)}${metric.name === 'CLS' ? '' : 'ms'}`,
        rating: metric.rating
    });
}

onCLS(sendToLeptos);
onINP(sendToLeptos);
onLCP(sendToLeptos);
onFCP(sendToLeptos);
onTTFB(sendToLeptos);

console.log('✅ Web Vitals initialized');