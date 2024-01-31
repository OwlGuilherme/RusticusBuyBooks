import scrapy
import json
import sqlite3
from scrapy.exceptions import CloseSpider

class BretasSpider(scrapy.Spider):
    name = "bretas"
    allowed_domains = ["bretas.com.br"]

    def __init__(self):
        super().__init__()
        self.setup_database()

    def setup_database(self):
        try:
            conn = sqlite3.connect('bretas.db')
            cursor = conn.cursor()

            # Criação da tabela de categorias
            cursor.execute('''CREATE TABLE IF NOT EXISTS categorias (
                                id_categoria INTEGER PRIMARY KEY,
                                nome_categoria TEXT UNIQUE)''')

            # Criação da tabela de produtos
            cursor.execute('''CREATE TABLE IF NOT EXISTS produtos (
                                id_produto INTEGER PRIMARY KEY,
                                nome_produto TEXT,
                                preco TEXT,
                                id_categoria INTEGER,
                                FOREIGN KEY (id_categoria) REFERENCES categorias(id_categoria))''')

            conn.commit()
            conn.close()
        except sqlite3.Error as error:
            print("Erro ao criar o banco de dados:", error)
            raise CloseSpider('Erro ao criar o banco de dados')

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

            # Inserir produto no banco de dados
            self.inserir_produto(categoria, nome, preco)

        # Adiciona as informações dos produtos desta página à lista de produtos da categoria
        if lista_produtos_pagina:
            yield {'categoria': categoria, 'produtos_pagina': lista_produtos_pagina}

            # Verifica se há mais páginas e continua navegando
            next_page_number = page_number + 1
            next_page_url = f"{response.url[:-1]}{next_page_number}"
            yield scrapy.Request(url=next_page_url, meta={'categoria': categoria, 'page_number': next_page_number}, callback=self.parse)

    def closed(self, reason):
        pass

    def inserir_produto(self, categoria, nome, preco):
        try:
            conn = sqlite3.connect('bretas.db')
            cursor = conn.cursor()

            # Verificar se a categoria já está na tabela
            cursor.execute('SELECT id_categoria FROM categorias WHERE nome_categoria = ?', (categoria,))
            categoria_id = cursor.fetchone()
            if categoria_id is None:
                cursor.execute('INSERT INTO categorias (nome_categoria) VALUES (?)', (categoria,))
                categoria_id = cursor.lastrowid
            else:
                categoria_id = categoria_id[0]

            # Inserir produto na tabela de produtos
            cursor.execute('INSERT INTO produtos (nome_produto, preco, id_categoria) VALUES (?, ?, ?)', (nome, preco, categoria_id))

            conn.commit()
            conn.close()
        except sqlite3.Error as error:
            print("Erro ao inserir produto:", error)

