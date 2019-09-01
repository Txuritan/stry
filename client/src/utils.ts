export function humanize(n: number) {
    let str = n.toString().split('.');

    str[0] = str[0].replace(/(\d)(?=(\d\d\d)+(?!\d))/g, '$1' + ',');

    return str.join('.');
}