from setuptools import setup, find_packages

setup(
    name='spacesurvival',
    author='Arne Beer, Rafael Epplée',
    author_email='contact@arne.beer',
    version='0.1.0',
    description='Spacesurvival',
    install_requires=[
        'flask>=0.12',
        'flask-security>=3.0.0',
        'flask-sqlalchemy>=2.2',
        'flask-marshmallow>=0.8',
        'marshmallow-sqlalchemy>=0.12.1',
        'webargs>=1.8.1',
        'sqlalchemy>=1.1.13',
        'sqlalchemy-utils>=0.32.16',
        'pytest',
    ],
    classifiers=[
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.5',
        'Environment :: Console'
    ],
    packages=find_packages(),
    entry_points={
        'console_scripts': [
            'spacesurvival=server:run'
        ]
    })
