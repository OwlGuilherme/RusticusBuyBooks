from typing import Any, Iterable
import scrapy
import json


class BretasSpider(scrapy.Spider):
    name = "bretas"
    allowed_domains = ["bretas.com.br"]

    def start_requests(self):
        with open("links.json") as file:
            links = json.load(file)

        for categoria in links:
            url_base = categoria['link']
            yield scrapy.Request(url=f"{url_base}1", meta={'categoria': categoria['categoria'], 'page_number': 1}, callback=self.parse)

    def parse(self, response):
        categoria = response.meta['categoria']
        page_number = response.meta['page_number']

        # Extrai informações dos produtos da página
        produtos = response.xpath('//article[contains(@class, "vtex-product-summary-2-x-element")]')
        lista_produtos_pagina = []

        for produto in produtos:
            nome = produto.xpath('.//h2[contains(@class, "bretas-bretas-components-0-x-ProductName")]/text()').get()
            preco = produto.xpath('.//h5[contains(@class, "bretas-bretas-components-0-x-crmDiscount")]/text()').get()
            lista_produtos_pagina.append({'nome': nome, 'preco': preco})

        # Adiciona as informações dos produtos desta página à lista de produtos da categoria
        if lista_produtos_pagina:
            yield {'categoria': categoria, 'produtos_pagina': lista_produtos_pagina}

            # Verifica se há mais páginas e continua navegando
            next_page_number = page_number + 1
            next_page_url = f"{response.url[:-1]}{next_page_number}"
            yield scrapy.Request(url=next_page_url, meta={'categoria': categoria, 'page_number': next_page_number}, callback=self.parse)

    def closed(self, reason):
        # Após o término do spider, combina todas as informações coletadas em um único arquivo JSON
        all_data = {}
        with open("output.json", "w") as json_file:
            for line in open("output.json"):
                data = json.loads(line)
                categoria = data['categoria']
                produtos_pagina = data['produtos_pagina']
                if categoria not in all_data:
                    all_data[categoria] = []
                all_data[categoria].extend(produtos_pagina)

            json.dump(all_data, json_file, ensure_ascii=False, indent=2)
